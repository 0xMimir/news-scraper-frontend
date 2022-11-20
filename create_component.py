#!/bin/env python3
import sys
import os
import stringcase

components_dir = "src/components/"

def get_name(name):
    component_name = stringcase.camelcase(name)
    component_name = component_name.replace(component_name[0], component_name[0].upper(), 1)
    return component_name


def get_component_template(name):
    return f"""use yew::{{function_component, html}};

#[function_component({get_name(name)})]
pub fn {name}() -> Html {{
    html!{{

    }}
}}"""


component = os.path.join(components_dir, sys.argv[1])
if os.path.isdir(component):
    print("Component already exists")
    exit()

print(f"Creating new component in {component}")
os.mkdir(component)

mod_file = os.path.join(component, "mod.rs")
open(mod_file, 'w').write("mod component;\npub use component::*;")

mod_file = os.path.join(component, "component.rs")
open(mod_file, 'w').write(get_component_template(sys.argv[1]))

mod_file = os.path.join(component, "style.scss")
open(mod_file, 'w').close()

append = open(os.path.join(components_dir, 'style.scss'), 'a')
append.write('\n@import "{}/style.scss";'.format(sys.argv[1]))

append = open(os.path.join(components_dir, 'mod.rs'), 'a')
append.write(
    '\n\nmod {};\npub use {}::{};'.format(
        sys.argv[1],
        sys.argv[1],
        get_name(sys.argv[1])
    )
)
