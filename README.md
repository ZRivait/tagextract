# tagextract

Extracts and inserts metadata tags from FLAC files. Writes to and reads from a text file for easy editing using your favourite text editor.

Usage:
tagextract [operations] [options] [format]

Operations:
extract: pull tags from the files in the current directory and print them to a file formatted by the format specifier
insert: pull tags from a text file (defaults to tags.txt in the same folder) and insert them into the files
	in the current directory with fields being based on the format specifier
print: pulls tags from the same text file but prints out the changes that are to be made and does nothing else

[format] is the format specifier. It uses % delimited tags given as a string. Each tag must be delimited by a character from the next tag. 

Examples:
%number%. %title%
%artist% - %title%
%disc%.%number%. %artist%:%title%

Supported Tags:
artist
title
album
albumartist
tracknumber
discnumber
genre
date
comment

Pass the --unsupported-tags option to use any tag you want

Options:

Shared:

--out, -o: Specifies the file to read from or write to. Defaults to tags.txt
