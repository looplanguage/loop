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
def AddTest(file_loc, answer):
    tests.append(Test("./tests/"+file_loc, answer))

def HasSucceeded(stdout, stderr, answer):
    output = stdout.decode(ENCODER)
    error = stderr.decode(ENCODER)
    if error:
        return False
    return output.strip() == answer

def RunTests():
    # Finds the executable regardless of platform
    exe = distutils.spawn.find_executable(BUILD)
    output = "End2End Test Results:\n"
    haveFailed = 0
    testCount = 0
    for test in tests:
        try:
            process = subprocess.Popen([exe, test.file_loc], stdout=subprocess.PIPE, stderr=subprocess.PIPE)
            # After 3 minutes (180 seconds) the program will crash, to prevent eternal loops
            stdout, stderr = process.communicate(timeout=180) 
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
        testCount += 1

    output += "\nTotal: {} - Failed: {} - Succeeded: {}\n".format(testCount, haveFailed, testCount - haveFailed)
    return (output, haveFailed>0)