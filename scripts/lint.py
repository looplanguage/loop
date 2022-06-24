import os
import subprocess

expected_cargo_version = "1.60"
crates_to_test = ["..", "picasso", "sanzio", "vinci"]
src_dir = "src/"
root_dir = os.getcwd()

# Check if cargo matches expected version
try:
    version = subprocess.check_output(["cargo", "version"]).decode("utf-8")
    if not version.startswith(f"cargo"):
        print(f"Wrong Cargo version detected. expected={expected_cargo_version}. got={version}")
        exit(1)
except subprocess.CalledProcessError:
    print("Unable to find Cargo!")


def lint_directory(directory):
    os.chdir(directory)

    try:
        subprocess.check_output(["cargo", "clippy", "--", "-D", "warnings"])
        subprocess.check_output(["cargo", "fmt", "--all"])
    except subprocess.CalledProcessError:
        exit(1)

    os.chdir(root_dir)


for crate in crates_to_test:
    lint_directory(src_dir + crate)
