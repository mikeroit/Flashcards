# Flashcards
This application is a terminal-based flashcards application written in Rust Language
The application is compatible with study sets exported from Quizlet

# Installation (Linux only... for now)
To install this app, you need to build it from source. Make sure you have Rust and Cargo installed on your machine. 
For most linux users, this can be done with two commands in the terminal:
`curl https://sh.rustup.rs -sSf | sh`
`sudo apt-get install cargo`

Installing the application is simple. First, clone this repository to your local machine:
`git clone https://github.com/mikeroit/Flashcards`

Next, move into the repository, and run the installation script:
`cd Flashcards`
`sh install.sh`

# Usage
To use the app, open a terminal and run the flashcards command:
`flashcards [<filenames>...]`

For example, given two study set files: *set-a.txt*, *set-b.txt*
run the command:
`flashcards set-a.txt set-b.txt`

# Flashcard File Formats
Proper flashcard files will be formatted as follows:
 * Each line in the file will represent an individual flashcard
 * Terms and definitions are seperated by the '-' character

# Quizlet compatablility 
To use an existing study set from Quizlet, open the study set in your browser, and select the 'more' button (button with 3 dots below the name of the study set)
Choose 'Export'
Under the column 'Between term and definition', specify the '-' character
Under the column 'Between rows', specify the 'New line' option
Now you can copy/paste the output to a new text file.


