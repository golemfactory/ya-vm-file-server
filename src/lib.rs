//! Filesystems core using 9P2000.L protocol, an extended variant of 9P from Plan 9.
//!
//! 9P protocol is originally developed for Plan 9 distributed OS.
//! As it's extendable and suitable for filesystems 9P is ported to Linux.
//! However, 9P protocol lacks Linux or Unix specific features,
//! which is the problem for developing serious filesystems.
//!
//! 9P2000.L is an extended variant protocol of 9P for Linux.
//! It has Linux specific features and is supported by Linux kernel 9P module.
//!
//! rs9p is a core to develop 9P2000.L virtual filesystems in Rust.
//! All you have to do is to implement `Filesystem` trait.

use std::sync::Arc;

use crate::core::srv::srv_async_inproc;
use crate::{core::attributes_cache::VirtualAttributesProvider, implementation::unpfs::Unpfs};
use tokio::io::DuplexStream;
use tokio::sync::Mutex;

#[macro_use]
pub mod core;
pub mod implementation;

pub struct InprocServer {
    filesystem: Unpfs,
}

impl InprocServer {
    pub fn new(mount_point: &str) -> Self {
        Self {
            filesystem: Unpfs {
                realroot: mount_point.into(),
                vap: Arc::new(Mutex::new(VirtualAttributesProvider::new())),
            },
        }
    }

    /// Attaches to the 9p server,
    /// Returns client stream to write requests, and read responses
    /// To detach simply drop the stream
    pub fn attach_client(&self) -> DuplexStream {
        const MAX_MESSAGE_SIZE: usize = 1024*1024;
        let (client, server) = tokio::io::duplex(MAX_MESSAGE_SIZE);

        tokio::spawn(srv_async_inproc(self.filesystem.clone(), server));

        client
    }
}

#[cfg(test)]
mod tests {

    use ::core::panic;
    use std::sync::Once;

    use bytes::{Buf, BufMut};
    use futures::{Future, SinkExt};
    use log::LevelFilter;
    use log4rs::{
        append::console::ConsoleAppender,
        config::{Appender, Root},
        Config,
    };
    use tokio::io::{ReadHalf, WriteHalf};
    use tokio_stream::StreamExt;
    use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};

    use crate::core::{
        fcall::{Fcall, Msg, QidType, NOTAG},
        serialize,
    };

    use super::*;

    /// Creates high level communication adapter for the 9P FS
    struct FSAdapter {
        msg_reader: FramedRead<ReadHalf<DuplexStream>, LengthDelimitedCodec>,
        msg_writer: FramedWrite<WriteHalf<DuplexStream>, LengthDelimitedCodec>,
    }

    impl FSAdapter {
        async fn send(&mut self, msg: &Msg) -> anyhow::Result<()> {
            let mut writer = bytes::BytesMut::with_capacity(65535).writer();
            serialize::write_msg(&mut writer, &msg).unwrap();

            self.msg_writer
                .send(writer.into_inner().freeze())
                .await
                .map_err(|e| anyhow::anyhow!("Failed sending the request {e}"))
        }

        async fn receive(&mut self) -> anyhow::Result<Msg> {
            if let Some(bytes) = self.msg_reader.next().await {
                let bytes = bytes?;
                return serialize::read_msg(&mut bytes.reader())
                    .map_err(|e| anyhow::anyhow!("Failed parsing the message: {e}"));
            }

            Err(anyhow::anyhow!("Reader stream is broken"))
        }

        fn new(server: &InprocServer) -> Self {
            let client = server.attach_client();

            let (reader, writer) = tokio::io::split(client);

            let framedread = LengthDelimitedCodec::builder()
                .length_field_offset(0)
                .length_field_length(4)
                .length_adjustment(-4)
                .little_endian()
                .new_read(reader);

            let framedwrite = LengthDelimitedCodec::builder()
                .length_field_offset(0)
                .length_field_length(4)
                .length_adjustment(-4)
                .little_endian()
                .new_write(writer);

            Self {
                msg_reader: framedread,
                msg_writer: framedwrite,
            }
        }
    }

    /// Boilerplate needed for setup testcases
    fn setup() {
        static START: Once = Once::new();
        START.call_once(|| {
            let stdout = ConsoleAppender::builder().build();

            let config = Config::builder()
                .appender(Appender::builder().build("stdout", Box::new(stdout)))
                .build(Root::builder().appender("stdout").build(LevelFilter::Debug))
                .unwrap();

            let _handle = log4rs::init_config(config).unwrap();
        });
    }

    async fn run_test(test: impl Future<Output = ()>) {
        setup();
        test.await;
        // teardown()
    }

    #[tokio::test]
    /// Create Inproc server, use returned endpoint to send "version" request, expect correct response
    async fn can_connect_to_the_inproc_server() {
        run_test(async {
            let temp_dir = tempdir::TempDir::new("can_connect_to_the_inproc_server").unwrap();

            let temp_dir_string = temp_dir.path().as_os_str().to_str().unwrap();
            let srv = InprocServer::new(temp_dir_string);

            let mut fs_adapter = FSAdapter::new(&srv);

            let request = Msg {
                tag: NOTAG,
                body: Fcall::Tversion {
                    msize: 8,
                    version: "9P2000.L".to_string(),
                },
            };

            fs_adapter.send(&request).await.unwrap();

            let msg = fs_adapter.receive().await.unwrap();

            assert_eq!(
                msg,
                Msg {
                    tag: NOTAG,
                    body: Fcall::Rversion {
                        msize: 8,
                        version: "9P2000.L".to_string()
                    }
                }
            );
        })
        .await
    }

    #[tokio::test]
    /// One 9P server can have many clients
    async fn can_attach_more_than_one_client() {
        run_test(async {
            let temp_dir = tempdir::TempDir::new("can_attach_more_than_one_client").unwrap();

            let temp_dir_string = temp_dir.path().as_os_str().to_str().unwrap();
            let srv = InprocServer::new(temp_dir_string);

            let request = Msg {
                tag: NOTAG,
                body: Fcall::Tversion {
                    msize: 8,
                    version: "9P2000.L".to_string(),
                },
            };

            let expected_response = Msg {
                tag: NOTAG,
                body: Fcall::Rversion {
                    msize: 8,
                    version: "9P2000.L".to_string(),
                },
            };

            let mut fs_adapter1 = FSAdapter::new(&srv);

            {
                // Second adapter lives shorter than first one
                let mut fs_adapter2 = FSAdapter::new(&srv);

                // They can co-exist
                fs_adapter1.send(&request).await.unwrap();
                fs_adapter2.send(&request).await.unwrap();

                assert_eq!(fs_adapter2.receive().await.unwrap(), expected_response);
                assert_eq!(fs_adapter1.receive().await.unwrap(), expected_response);
            }

            // Second connection dropped, first still works
            fs_adapter1.send(&request).await.unwrap();
            assert_eq!(fs_adapter1.receive().await.unwrap(), expected_response);
        })
        .await
    }

    #[tokio::test]
    /// Do simple operation on the server
    async fn can_create_a_file() {
        run_test(async {
            let temp_dir = tempdir::TempDir::new("can_create_a_file").unwrap();

            let temp_dir_string = temp_dir.path().as_os_str().to_str().unwrap();
            let srv = InprocServer::new(temp_dir_string);

            let mut fs_adapter = FSAdapter::new(&srv);

            // Start a session by sending version
            fs_adapter
                .send(&Msg {
                    tag: NOTAG,
                    body: Fcall::Tversion {
                        msize: 8,
                        version: "9P2000.L".to_string(),
                    },
                })
                .await
                .unwrap();

            assert_eq!(
                fs_adapter.receive().await.unwrap(),
                Msg {
                    tag: NOTAG,
                    body: Fcall::Rversion {
                        msize: 8,
                        version: "9P2000.L".to_string(),
                    },
                }
            );

            // Attach to the endpoint
            let mut msg_id = 0;
            fs_adapter
                .send(&Msg {
                    tag: msg_id,
                    body: Fcall::Tattach {
                        fid: 1,
                        afid: u32::MAX,
                        uname: "".to_string(),
                        aname: "".to_string(),
                        n_uname: 0,
                    },
                })
                .await
                .unwrap();

            match fs_adapter.receive().await.unwrap() {
                Msg {
                    tag,
                    body: Fcall::Rattach { qid },
                } => {
                    assert_eq!(tag, msg_id);
                    assert_eq!(qid.typ, QidType::DIR);
                }
                other => panic!("Invalid response {other:?}"),
            };

            const FILE_FID: u32 = 2;

            // Create new fid
            msg_id += 1;
            fs_adapter
                .send(&Msg {
                    tag: msg_id,
                    body: Fcall::Twalk {
                        fid: 1,
                        newfid: FILE_FID,
                        wnames: vec![],
                    },
                })
                .await
                .unwrap();

            match fs_adapter.receive().await.unwrap() {
                Msg {
                    tag,
                    body: Fcall::Rwalk { wqids },
                } => {
                    assert_eq!(tag, msg_id);
                    assert_eq!(wqids, vec![]);
                }
                other => panic!("Invalid response {other:?}"),
            }

            // Create a file using the fid
            msg_id += 1;
            fs_adapter
                .send(&Msg {
                    tag: msg_id,
                    body: Fcall::Tlcreate {
                        fid: FILE_FID,
                        name: "test.txt".to_string(),
                        flags: 35137,
                        mode: 33188,
                        gid: 1000,
                    },
                })
                .await
                .unwrap();

            match fs_adapter.receive().await.unwrap() {
                Msg {
                    tag,
                    body: Fcall::Rlcreate { qid, iounit: _ },
                } => {
                    assert_eq!(tag, msg_id);
                    assert_eq!(qid.typ, QidType::FILE);
                }
                other => panic!("Invalid response {other:?}"),
            }
        })
        .await
    }
}
