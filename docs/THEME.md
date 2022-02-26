# Theme

## Syntax

A theme string must be double quoted and respect the following rules:

- strings must be quoted with single quotation mark like this `` `hello world !` ``
- to specify text color you must use this marker `&[hex_color]`, replace `hex_color` with the corresponding rgb encoded hex color. For example use `&[#FF00FF]` for magenta.
- for variables use the following syntax `{{my_var}}` where my_var is a variable provided by the terminal. See below for an exhaustive list.

A theme is located on the vsh config file under the `[prompt]` section and on the `theme=` line

## Examples

Because exemples are better:

```
"&[#7393B3]`[`{{current_dir}}`] `"
```

would produce the following result:

![exmaple1](https://user-images.githubusercontent.com/29127537/155847852-5908d8a5-ba9e-4745-bd55-3627cc860374.png)

Here is another example you could try to use:

```
"&[#D8BFD8]{{username}}`@`{{hostname}}&[#7393B3]` `{{current_dir}}&[#FFFFFF]`$ `"
```

Feel free to start your own themes from these example.

## Variables

Here is an exhaustive list of all variables in vsh

### Session

| Variable Name |              Description              |    Exemple |
| ------------- | :-----------------------------------: | ---------: |
| desktop_env   | Gives the current desktop environment |      gnome |
| real_name     |   Returns the real name of the user   |      linus |
| username      |       Returns the unix username       |      linus |
| device_name   |         Gives the device name         |     server |
| hostname      |         Returns the hostname          | vsh_server |
| platform      | Returns the platform: windows / linux |      linux |
| distribution  |        Returns the unix distro        |       arch |

### Date

#### Utc

| Variable Name |                             Description                             | Exemple |
| ------------- | :-----------------------------------------------------------------: | ------: |
| utc_ss        |                  Returns seconds with utc timezone                  |      23 |
| utc_mm        |                  Returns minutes with utc timezone                  |      45 |
| utc_hh12      |            Returns hours from 1 to 12 with utc timezone             |       1 |
| utc_AMPM      |                   Returns AM/PM with utc timezone                   |      PM |
| utc_hh24      |            Returns hours from 1 to 24 with utc timezone             |      13 |
| utc_DD        |               Returns day of month with utc timezone                |      20 |
| utc_MM        |                  Returns months with utc timezone                   |      07 |
| utc_YYYY      |                   Returns years with utc timezone                   |    2022 |
| utc_ord       | Returns ordinal (days from the start of the year) with utc timezone |     169 |
| utc_wday      |                 Returns week day with utc timezone                  |      Me |

#### Local

| Variable Name |                              Description                              | Exemple |
| ------------- | :-------------------------------------------------------------------: | ------: |
| loc_ss        |                  Returns seconds with local timezone                  |      23 |
| loc_mm        |                  Returns minutes with local timezone                  |      45 |
| loc_hh12      |            Returns hours from 1 to 12 with local timezone             |       1 |
| loc_AMPM      |                   Returns AM/PM with local timezone                   |      PM |
| loc_hh24      |            Returns hours from 1 to 24 with local timezone             |      13 |
| loc_DD        |               Returns day of month with local timezone                |      20 |
| loc_MM        |                  Returns months with local timezone                   |      07 |
| loc_YYYY      |                   Returns years with local timezone                   |    2022 |
| loc_ord       | Returns ordinal (days from the start of the year) with local timezone |     169 |
| loc_wday      |                 Returns week day with local timezone                  |      Me |

## Directory

| Variable Name |       Description       |         Exemple |
| ------------- | :---------------------: | --------------: |
| current_dir   | Returns the current dir | ~/Documents/vsh |

## Exit Codes

| Variable Name |                             Description                             | Exemple |
| ------------- | :-----------------------------------------------------------------: | ------: |
| terminated    | Returns false or true according to the previous program terminaison |    true |
| exit_code     |        Return exit code of the previous program terminaison         |       1 |
