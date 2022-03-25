import distutils.spawn
import subprocess
import sys
from dataclasses import dataclass

#BUILD = "./loop"
BUILD = "./target/release/loop"
ENCODER = "utf-8"

@dataclass
class Test:
    file_loc: str
    answer: int

tests = []
def AddTest(file_loc, answer):
    tests.append(Test(file_loc, answer))

def HasSucceeded(stdout, stderr, answer):
    output = stdout.decode(ENCODER)
    error = stderr.decode(ENCODER)
    if stderr and stdout:
        return False
    elif stderr:
        return False

    return int(stdout) == answer

def RunTests():
    exe = distutils.spawn.find_executable(BUILD)
    output = "Integration Test Results:\n"
    haveFailed = 0
    test_count = 0
    for test in tests:
        try:
            process = subprocess.Popen([exe, test.file_loc], stdout=subprocess.PIPE, stderr=subprocess.PIPE)
            stdout, stderr = process.communicate(timeout=180) # After 3 minutes the program will crash
            if HasSucceeded(stdout, stderr, test.answer):
                output += "    > {}   -->   SUCCESS\n".format(test.file_loc.split('/')[-1])
            else:
                output += "    > {}   -->   FAILED\n".format(test.file_loc.split('/')[-1])
                haveFailed += 1
        except subprocess.TimeoutExpired:
            process.kill()
            stdout, stderr = process.communicate()
            output += "    > {}   -->   FAILED [TIME EXPIRED]\n".format(test.file_loc.split('/')[-1])
            haveFailed += 1
        test_count += 1

    output += "\nTotal: {} - Failed: {} - Succeeded: {}\n".format(test_count, haveFailed, test_count - haveFailed)
    return (output, haveFailed>0)