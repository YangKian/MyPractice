#!/bin/sh

# Introduction:The script is used to view, add or delete a setting in a configuration file. Modify the setting 
# which exists is not allowed.


# The function uses to display the menu
show()
{
    echo "***MENU***"
    echo "1. Add a Setting\n2. Delete a Setting\n3. View a Setting\n4. View All Settings\nQ - Quit"
    echo "\nCHOICE:" 
}


# The function uses to add a setting
add()
{
    while :; do
	read -p "Enter setting (format: ABCD=abcd):" temp

	# If users use just simply presses the Enter/Return key, give them a warnning and ask users to input again.
	setting=${temp:-"setting"}
	[ "${setting}" = "setting" ] && echo "New setting not entered\n" && continue

	# Check if the input setting contatins a "=" sign
	if [ $( expr "${setting}" : ".*=.*" ) -eq 0 ]; then
	    echo "Invalid setting\n";
	    continue
	fi


	# If a setting contains a "=" sign, split the key and value, then display them and make sure key and value are not null.
	key=$( echo "${setting}" | cut -d "=" -f 1 )
	value=$( echo "${setting}" | cut -d "=" -f 2 )
	echo "The variable name of the setting is: ${key}\nThe variable value of the setting is: ${value}\n"
	[ -z "${key}" ] || [ -z "${value}" ] && echo "Invalid setting.\n" && continue
    
	# Make sure the key is invalid, which means it can not begin with a digit number.
	if [ $( expr "${key}" : "[0-9].*" ) -ne 0 ]; then
	    echo "Invalid setting. The first character of a variable name connot be a digit.\n"
	    continue
	fi

	# Check the key is not contains in the config file, then add the setting. If it has contained in the file, give a warnning to user and ask another input.
	if [ -z "$( grep ${key}= ./config.txt | cut -d "=" -f 1 )" ]; then
	    echo ${setting} >> ./config.txt && echo "New setting added.\n" && break 1
	else
	    echo "Variable exists. Changing the values of existing variables is not allowed.\n" && continue
	fi
    done
}


# The function uses to delete a setting.
delete()
{
    while :; do
	read -p "Enter variable name:" deleted

	# Make sure user input something as the variable name which need to be deleted.
	setting=${deleted:-"setting"}
	[ "${setting}" = "setting" ] && echo "Variable not entered\n" && continue

	# Make sure the input variable is contains in the file, then lists the variable and its value, do the operation users want.
	result=$( grep ${setting}= ./config.txt )
	if [ $? -ne 0 ]; then
	    echo "Variable does not exist.\n" && continue
        else
	    echo ${result}
	    read -p "Delete this setting (y/n)?" choice
	    if [ "${choice}" = "y" ]; then
		sed -i "/${setting}=/d" ./config.txt && echo "\nSetting deleted.\n" && break 1
	    else
		echo "\ncancel the delete option.\n" && break 1
	    fi
	fi
    done
}


# The function uses to show a setting
view()
{
    while :; do
	read -p "Enter variable name:" key

	# Make sure user input something as the variable name which need to show
	setting=${key:-"setting"}
	[ "${setting}" = "setting" ] && echo "Variable not entered\n" && continue

	# Check whether the variable is contained in the file, then display the result.
	result=$( grep ${setting}= ./config.txt )
	if [ $? -ne 0 ]; then
	    echo "Variable does not exist.\n" && continue
	else
	    echo ${result} && echo "Requested setting displayed above.\n" && break 1
	fi
    done
}


# The function uses to show all settings.
viewAll()
{
    cat ./config.txt
    echo "\n"
}


while :; do
    show
    read option
    case ${option} in
	"1")
	    add
	    ;;
	"2")
	    delete
	    ;;
	"3")
	    view
	    ;;
	"4")
	    viewAll
	    ;;
	"Q"|"q")
	    break
	    ;;
	*)
	    echo "Please input the right option [1/2/3/4/Q]\n"
	    ;;
    esac
done

