# Steps to run this project
1. ```git clone https://github.com/Dandandooo/RustRegex```
2. Open the project
3. Open ```test_cases.txt``` or make your own txt file with the same format
4. Format is as follows:
   1. First line is the regex
   2. Following lines are strings to test against the regex
   3. Each line is ```<test_string> <expected_output>```, delimited by a space
4. Save the file
5. In the terminal: ```cargo run```