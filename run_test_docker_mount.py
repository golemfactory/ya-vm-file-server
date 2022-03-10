import os
import subprocess
import time

build_docker_command = f"docker build . -t ya-vm-file-server"
print(f"Running command: {build_docker_command}")
build_docker = subprocess.Popen(build_docker_command, shell=True)
build_docker.communicate()

run_server_commmand = f'cargo run --features="build-binary" -- --mount-point server-mount --network-address 127.0.0.1:12397'
print(f"Running command: {run_server_commmand}")
run_server = subprocess.Popen(run_server_commmand, shell=True)

run_compose_command = f"docker-compose -f docker-compose-test-external.yml up"
print(f"Running command: {run_compose_command}")
docker_compose = subprocess.Popen(run_compose_command, shell=True)

time.sleep(20.0)

run_socat_command = f"socat-windows\socat.exe TCP4:127.0.0.1:12397 TCP4:127.0.0.1:12398"
print(f"Running command: {run_socat_command}")
docker_compose = subprocess.Popen(run_socat_command, shell=True)


