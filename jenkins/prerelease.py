import distutils.spawn

import logging
import boto3
from botocore.exceptions import ClientError
import os

import toml
import platform

def upload():
    f = open("./Cargo.toml", "r")

    file = toml.load("./Cargo.toml")

    build = os.getenv("BUILD_NUMBER")

    if build is None:
        build = "0"

    filename_to_upload = platform.system() + "_" + platform.machine() + "_loop_" + file["package"]["version"] + "-" + build
    file_to_upload = distutils.spawn.find_executable("./target/release/loop")

    filename, file_extension = os.path.splitext(file_to_upload)

    filename_to_upload = filename_to_upload + file_extension

    print(file_to_upload)

    s3_client = boto3.client('s3')
    try:
        response = s3_client.upload_file(file_to_upload, "loopartifacts", "Prerelease/" + filename_to_upload)
    except ClientError as e:
        logging.error(e)
        return False
    return True

if __name__ == '__main__':
    upload()
