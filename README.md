The PoW algorithm used in this code is SHA-256. The sha2 library in Rust is utilized to create a digest of a challenge string that is made up of the client's IP address and a random nonce. This digest is then converted to a hexadecimal string, and the first DIFFICULTY number of characters are checked to see if they are all zeroes. If they are, the client has successfully completed the PoW challenge and is permitted to proceed with their request.

