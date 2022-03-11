#!/bin/bash
set -x # echo on

echo "Sleeping 5 seconds before starting"
sleep 5s
cd /home/dock/ya-vm-file-server
MOUNT_POINT="./mnt_tests"
echo "Mounting filesystem from external server"
sudo mount -t 9p -o version=9p2000.L,trans=tcp,debug=0x04,port=7878,uname=testuser `getent hosts test-vm-server | awk '{ print $1 }'` $MOUNT_POINT
echo "Confirm mount"
if mountpoint -q -- $MOUNT_POINT; then
  printf '%s\n' "client_fs is a mount point"
  ls -la $MOUNT_POINT
  printf '%s\n' "running tests"
  cargo +nightly test
  sleep 1000s
fi
