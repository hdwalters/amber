#!/usr/bin/env bash
# Written in [Amber](https://amber-lang.com/)
# version: 0.3.5-alpha
# date: 2024-12-03 22:33:14
# #!/usr/bin/env amber
set_ref__0_v0() {
    local __AMBER_ARRAY_numbers="$1[@]"
    local __AMBER_REF_numbers=$1
    echo "In fun before set: ${!__AMBER_ARRAY_numbers}"
    __AMBER_ARRAY_0=(4 5 6)
    eval "${__AMBER_REF_numbers}=(\"\${__AMBER_ARRAY_0[@]}\")"
    echo "In fun after set: ${!__AMBER_ARRAY_numbers}"
}

__AMBER_ARRAY_1=(1 2 3)
numbers=("${__AMBER_ARRAY_1[@]}")
echo "In main before call: ${numbers[@]}"
set_ref__0_v0 numbers
__AF_set_ref0_v0__12_5="$__AF_set_ref0_v0"
echo "$__AF_set_ref0_v0__12_5" >/dev/null 2>&1
echo "In main after call: ${numbers[@]}"
