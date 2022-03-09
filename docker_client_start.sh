echo "Sleeping 5 seconds before starting"
sleep 5s
echo "Mounting filesystem from external server"
echo mount -t 9p -o version=9p2000.L,trans=tcp,debug=0x04,port=7878,uname=testuser `getent hosts test-vm-server | awk '{ print $1 }'` ./client_fs/
mount -t 9p -o version=9p2000.L,trans=tcp,debug=0x04,port=7878,uname=testuser `getent hosts test-vm-server | awk '{ print $1 }'` ./client_fs/
echo "Confirm mount"
if mountpoint -q -- "client_fs"; then
  printf '%s\n' "client_fs is a mount point"
  ls -la ./client_fs
  sleep 1000s
fi
