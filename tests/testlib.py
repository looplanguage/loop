import distutils.spawn
import subprocess
from dataclasses import dataclass

#BUILD = "./loop"
BUILD = "./target/release/loop"
ENCODER = "utf-8"

@dataclass
class Test:
    file_loc: str
    answer: str

tests = []
def add_test(file_loc, answer):
    tests.append(Test("./tests/"+file_loc, answer))

def has_succeeded(stdout, stderr, answer):
    output = stdout.decode(ENCODER)
    error = stderr.decode(ENCODER)
    if error:
        return False
    return output.strip() == answer

def run_tests():
    # Finds the executable regardless of platform
    exe = distutils.spawn.find_executable(BUILD)
    output = "End2End Test Results:\n"
    have_failed = 0
    test_count = 0
    for test in tests:
        try:
            process = subprocess.Popen([exe, test.file_loc], stdout=subprocess.PIPE, stderr=subprocess.PIPE)
            # After 3 minutes (180 seconds) the program will crash, to prevent eternal loops
            stdout, stderr = process.communicate(timeout=180) 
            if has_succeeded(stdout, stderr, test.answer):
                output += "    > {}   -->   SUCCESS\n".format(test.file_loc.split('/')[-1])
            else:
                output += "    > {}   -->   FAILED\n".format(test.file_loc.split('/')[-1])
                have_failed += 1
        except subprocess.TimeoutExpired:
            process.kill()
            stdout, stderr = process.communicate()
            output += "    > {}   -->   FAILED [TIME EXPIRED]\n".format(test.file_loc.split('/')[-1])
            have_failed += 1
        test_count += 1

    output += "\nTotal: {} - Failed: {} - Succeeded: {}\n".format(test_count, have_failed, test_count - have_failed)
    return (output, have_failed>0)