#!/bin/bash

# Get the program name from the first argument
program_name="nu"

# Check if a program name is provided
if [ -z "$program_name" ]; then
	echo "Please provide the name of the program to check."
	exit 1
fi

# Detect the operating system
os_name=$(uname -s)

# Check software installation based on OS
case "$os_name" in
# Add more OS checks as needed
"Darwin") # macOS
	# Check Homebrew first
	if $program_name --version &>/dev/null; then
		echo "nushell is installed with Homebrew."
	else
		# Then check Applications and command-line tools
		if [ -d "/Applications/$program_name.app" ]; then
			echo "$program_name is installed."
		else
			if command -v "$program_name" &>/dev/null; then
				echo "$program_name is installed (might not be from Homebrew)."
			else
				echo "$program_name is not installed."
			fi
		fi
	fi
	;;
"Linux") # Linux
	# Use package manager (replace with your specific command)
	if dpkg -l "$program_name" | grep -i installed &>/dev/null; then
		# echo "$program_name is installed."
		echo true
	else
		if command -v "$program_name" &>/dev/null; then
			echo true
		else
			echo false
		fi
	fi
	;;
*)
	echo "Unsupported operating system: $os_name"
	;;
esac
