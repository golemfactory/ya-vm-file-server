import os
import subprocess

build_docker_command = f"docker build . -t ya-vm-file-server"
print(f"Running command: {build_docker_command}")
build_docker = subprocess.Popen(build_docker_command, shell=True)
build_docker.communicate()

run_compose_command = f"docker-compose -f docker-compose.yml up"
print(f"Running command: {run_compose_command}")
docker_compose = subprocess.Popen(run_compose_command, shell=True)
