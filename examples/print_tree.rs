use libsmi_sys::*;
use std::os::raw::c_char;

fn main() {

    let mut ctx = SmiContext::init().expect("Context not opened");
    ctx.set_search_path("/usr/share/snmp/mibs:/usr/share/snmp/mibs/iana:/usr/share/snmp/mibs/ietf").expect("Failed to set path");
    ctx.load_module("RFC1213-MIB").expect("loaded module");

    let root = ctx.lookup_node("1").unwrap();
    print_children(root, "");
}

fn print_children(node: SmiNode, prefix: &str) {
    let mut datatype = match node.textual_convention() {
        Some(conv) => conv.name,
        None => "unknown type"
    };


    println!("{}{} ({})({}) -- {}", prefix, node.qualified_name(), node.kind, node.format, datatype);
    if datatype == "" {
        let t = &node.textual_convention().unwrap().basetype.to_string();
        println!("{}", t);
    }

    for n in node.children() {
        let child_prefix = prefix.to_owned() + "| ";
        print_children(n, &child_prefix);
    }
}