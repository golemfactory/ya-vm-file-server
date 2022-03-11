sudo nohup socat TCP4-LISTEN:7100 TCP4-LISTEN:12398 &
echo "Sleeping 5 seconds before starting"
sleep 5s
cd /home/dock/ya-vm-file-server
MOUNT_POINT="./mnt_tests"
echo "Mounting filesystem from external server"
echo sudo mount -t 9p -o version=9p2000.L,trans=tcp,debug=0x04,port=7100,uname=testuser 127.0.0.1 ./mnt_tests/
sudo mount -t 9p -o version=9p2000.L,trans=tcp,debug=0x04,port=7100,uname=testuser 127.0.0.1 ./mnt_tests/
echo "Confirm mount"
if mountpoint -q -- "mnt_tests"; then
  printf '%s\n' "client_fs is a mount point"
  ls -la ./mnt_tests
  printf '%s\n' "running tests"
  cargo +nightly test

fi

sleep 1000s