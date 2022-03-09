echo "Sleeping 5 seconds before starting"
sleep 5s
cd /ya-vm-file-server
echo "Mounting filesystem from external server"
echo mount -t 9p -o version=9p2000.L,trans=tcp,debug=0x04,port=7878,uname=testuser `getent hosts test-vm-server | awk '{ print $1 }'` ./mnt_tests/
mount -t 9p -o version=9p2000.L,trans=tcp,debug=0x04,port=7878,uname=testuser `getent hosts test-vm-server | awk '{ print $1 }'` ./mnt_tests/
echo "Confirm mount"
if mountpoint -q -- "mnt_tests"; then
  printf '%s\n' "client_fs is a mount point"
  ls -la ./mnt_tests
  printf '%s\n' "running tests"
  cargo test
  sleep 1000s
fi
