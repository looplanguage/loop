# Imports the test library
from testlib import RunTests, AddTest

def main():
    # Add your tests right here.
    # AddTest( FILE LOCATION OF LOOP FILE, EXPECTED ANSWER )
    # NOTE: The expected answer always needs to be a integer
    AddTest("test_comments.loop", 3)
    AddTest("test_import_export.loop", 8)

    # output, is the generated report of all the tests
    # hasFailed, a boolean:
    #   > True: One or more test have failed.
    #   > False: All tests passed
    output, hasFailed = RunTests()

    # Asserts if 'hasFailed' is false, otherwise prints a red failure message
    assert not hasFailed, output + "\033[91mIntegration Tests Have Failed...\033[0m"

    # Prints a green success message
    print(output + "\033[92mIntegration Tests Have Succeeded...\033[0m")

if __name__ == "__main__":
    main()