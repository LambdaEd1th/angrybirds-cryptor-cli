# Bad Piggies Cryptor

* Author: ed1th
* Version: 0.1.0

## Description
Angry Birds file cryptor<br>
Encrypt or decrypt **Angry Birds Classic | Rio | Seasons | Space | Friends | Star Wars | Star Wars II | Stella** game files

## Usage
`angrybirds-cryptor-cli <COMMAND>`

### Example
#### Decrypt Progress.dat file
`angrybirds-cryptor-cli decrypt -f native -g classic -i example.lua -o example.dec.lua`
#### Encrypt .contraption file
`angrybirds-cryptor-cli encrypt -f save -g seasons -i example.dec.lua -o example.lua`