# tagextract

Extracts and inserts metadata tags from FLAC files. Writes to and reads from a text file for easy editing using your favourite text editor.

Usage:
tagextract x [options] [format]
tagextract i [options] [format]

[format] is the format specifier. It uses % delimited tags given as a string. Each tag must be delimited by a character from the next tag. For x operations it affects the output and for i operations it affects the inserted tags.

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
