#!/bin/sh


# Introduction: The script is used to remove old C programs which are no longer wish to keep.
# When no arguments supplied, it will pick up each C program from current directory and lists 
# the first 10 lines, you can use these lines to determin whether the file is deleted or not. 
# If you supply arguments, it works on those files only.


# The function uses to list each file`s first 10 line and give a choice whether the file is to 
# delete or not.
fileClean()
{
    # Traverse all filtered files, check whether the file is still in the directory or not, then
    # do the operation.
    for i in $@; do
	echo "\nDisplaying first 10 lines of $i:\n"
	[ ! -f "$i" ] && echo "File $i does not exist." && continue
	head $i
        echo "\nDelete file $i? (y/n):"
        read choice
        if [ "$choice" = "y" ]; then
	    rm $i
	    echo "File $i deleted.\n"
        elif [ "$choice" = "n" ]; then
	    echo "File $i NOT deleted.\n"
        else
	    echo "You need to input y/n to determin whether a file is deleted or not.\n"
	    continue
        fi
    done
}


# Separate the case by checking the number of parameters.
if [ $# -lt 1 ]; then
    echo "This script removes C files which you no longer want to keep.\nHere are the C file(s) under the current directory:"
    
    # check if current directory contains .c files.
    # If it has, list the files name and then deal with the files, otherwise the program will return.
    filenames=$( ls *.c 2> /dev/null )
    if [ -z "${filenames}" ]; then
        echo "No C files found."
        return
    else
        echo $filenames
    fi
    fileClean $filenames
else 
    # Deal with the condition with parameters.
    echo "This script removes C files which you no longer want to keep.\nThe file(s) you want to delete is/are:"
    echo $@
    fileClean $@
fi
