# Needle in a YAML

A Rust program that parses a YAML file given by command line argument. Use the TUI to explore the YAML.
Useful in large YAML files with many tranlations, for instance.

# Overview

Explore your big YAML files by using this project. Use the TUI to put the keys and nested keys to get to the value you want.

Run this command

> cargo run -- name_of_the_yaml_file.yaml

Press "e" key to enter "e"dit mode, input the name of the keys no narrow down the sections of your YAML file. After you
input the key you want at the root level of the YAML hit enter. You will see on the TUI only the value associated to that file.

If there are more maps/dictionaries/objects/hashes nested withing that key, input the name of the subkey and hit enter, you will see
on the TUI only the value associated to that nested hash you selected. Continue until you find what you need.

At the bottom of the TUI you will find the rails command to use the translation. If you want to see a big YAML file with translations,
take a look at this one: https://github.com/edavis10/redmine/blob/master/config/locales/en.yml
