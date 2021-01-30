# Singh code book decypher
Some doodles and fiddling in Rust with samples from Simon Singh's "Code Book". I thoroughly enjoy reading his work and
at the example in chapter 1 I felt the itch to write some code in order to do the decoding. It all started unwrapping in
my head and since I like exercising in Rust I thought I'd give it a go.

# Functionality
Do 
```bash
cargo build
cargo run
```

Given an input text in upper case and a cipher in yaml mapping to lower case, it will decode the message which i believe
is supposed to be an encrypted message from Mary Queen of Scots.
- [x] Reads input text from [src/encrypted-letter.txt](src/encrypted-letter.txt)
- [x] Prints the text to your terminal
- [ ] No you cannot pass a different text yet
- [x] Reads the cipher from [src/cipher.yaml](src/cipher.yaml)
- [ ] And no, you cannot choose a different file name yet. I may get to this. Or this may be left as "an exercise to the reader"
- [x] Prints the cipher (i.e. the mapping from source chars to target chars) to your terminal
- [x] It will replace all character keys by the character values from the mapping
- [x] Prints the decoded letter to your terminal

That's it, enjoy.
