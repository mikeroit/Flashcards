#Build the application
echo 'Building flashcards app from source'
cargo build

#Copy the executable to /bin directory
echo 'Copying executable to /bin'
sudo cp ./target/debug/flashcards /bin/

echo 'Install complete'



