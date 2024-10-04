mod parent_child;
mod parent_child_id;
mod item_names;
mod dimension;
mod three_dim_value;
mod value_keys;
pub mod questions_and_answers;


use parent_child::*;
use parent_child_id::*;
use item_names::*;
use dimension::*;
use questions_and_answers::Questions;
use three_dim_value::*;
use value_keys::ValueKeys; 

fn main(){
    /*
    fn_test_push();
    
    let s = String::from("TestRef_");
    fn_test_ref(&s);
    // Because s was passed by reference we didn't transfer ownership, and it's still valid
    println!("{s}");
 */

    //fn_test_parent_child_id();


   // fn_test_parent_child();

    //fn_test_parent_child_ids();

    /*
    let mut item_names = ItemNames::build();
    match item_names.read_csv("./test_data/dimensions/cost_center_names.csv") {
        Ok(_x) => println!("Read item names ok"),
        Err(_e) => println!("read item names not ok")
    }

    let mut parent_child_tree:ParentChild = ParentChild::create_parent_child(-1);
    
    let mut parent_child_data: ParentChildIds = ParentChildIds::build();
    match parent_child_data.read_csv("./test_data/dimensions/cost_center_parent_child.csv") {
        Ok(_y) => println!("Read parent child ids ok"),
        Err(_e) => println!("read parent child ids not ok")
    }

    parent_child_tree.build(& mut parent_child_data);


    parent_child_tree.print_names("    ".to_string(),&mut item_names);
    */


    let mut cost_centers =  Dimension::new("Cost Centers".into());    
    cost_centers.load("./test_data/dimensions/cost_center_names.csv",
    "./test_data/dimensions/cost_center_parent_child.csv");    
    cost_centers.root_item.print_names(String::from("  "), & mut cost_centers.names);


    
    let mut periods = Dimension::new("Periods".into());
    periods.load("./test_data/dimensions/period_names.csv",
        "./test_data/dimensions/period_parent_child.csv");
    periods.root_item.print_names(String::from("  "), & mut periods.names);
    


    let mut costs = Dimension::new("Costs".into());
    costs.load("./test_data/dimensions/cost_names.csv",
        "./test_data/dimensions/cost_parent_child.csv");
    costs.root_item.print_names(String::from("  "), & mut costs.names);
    








    let mut value_list = ThreeDimValues{items: Vec::new(),_sorted:false};
    for p in periods.root_item.get_leaf_item_ids() {
        for c in cost_centers.root_item.get_leaf_item_ids() {        
            for t in costs.root_item.get_leaf_item_ids() {
                let xx = ValueKeys{keys:vec![p,c,t], length:3 };
                ThreeDimValues::add_item(&mut value_list, ThreeDimValue{key: xx, value: (c as f64 * 10.143)+ p as f64 + (t as f64*100.313)});            
            }
        }
    }

    
    let mut t: f64 = 0.0;
    for value in &value_list.items {
      //  println!("{value}");
        t = t + value.value;
    }

    println!("TOTAL: {t}");

        
    let mut dims = Vec::new();
    dims.push(cost_centers);
    dims.push(periods);
    dims.push(costs);

    let mut q = Questions{
        dimensions: dims,
        values: value_list
    };

    let a  = q.ask_question([21,22,23]);

    println!("{}",a);



}

fn fn_test_push(){
    let s = &mut String::from("HELLO");
    // can only modify 's' here as it is declared as mutable (mut)
    s.push_str(" world");
    println!("{s}");
    let l = s.len();
    println!("{l}");
}

fn fn_test_ref(s: &str){
    println!("{s}");
}

/*
fn fn_test_parent_child_id() {

    let pc = ParentChildIdPair{parent_id:1,child_id:2};
    let parent_id = pc.parent_id;
    let child_id = pc.child_id;
    println!("{parent_id},{child_id}");

    let mut v: Vec<ParentChildIdPair> = vec![];
    v.push(pc);
    v.push(ParentChildIdPair{parent_id:5,child_id:9});// test ordering
    v.push(ParentChildIdPair{parent_id:1,child_id:3});
    v.push(ParentChildIdPair{parent_id:1,child_id:4});
    v.push(ParentChildIdPair{parent_id:1,child_id:5});

    v.push(ParentChildIdPair{parent_id:5,child_id:6});
    v.push(ParentChildIdPair{parent_id:5,child_id:7});
    v.push(ParentChildIdPair{parent_id:5,child_id:8});
    let l = v.len();
    println!("Vector length is {l}");




    let found_item = v.get(2);
    // Value mut be unwrapped (use match)
    // let i = found_item.unwrap();
    // println!("found item is {i}");
    match found_item {
        Some(found_item) => println!("found item {found_item}"),
        None => println!("item not found")
    }

    // Test sorting and binary_search
    v.sort();
    for z in &v {
        println!("{z}");
    }
    let search_key = ParentChildIdPair{parent_id:5,child_id:6};
    let f = &v.binary_search(&search_key);
    match f {
        Ok(i) => {
            println!("FOUND AT INDEX: {i}");
            let found_item = v.get(*i); // dereference the pointer
            match found_item {
                Some(found_item) => println!("found : {found_item}"),
                None => println!("item not found")
            }

        },
        _ => println!("error")
    }
}
*/

/*
fn fn_test_parent_child_ids() {
    let mut ids = ParentChildIds::build();
    assert_eq!(ids.len(),0);
    ids.add_item(1,2);
    ids.add_item(1,3);
    ids.add_item(1,4);
    ids.add_item(2,5);
    assert_eq!(ids.len(),4);

    //assert_eq!(ids.get_item(1).child_id,3);

    ids.iter().for_each(|f| println!("pcid: `{f}"));

    let item_index = ids.get_index(ParentChildIdPair{parent_id:1,child_id:4}).unwrap();
    assert_eq!(item_index,2);

    match ids.get_item(item_index) {
        Some(item) => {
            assert_eq!(item.child_id,4);
        },
        None => assert_eq!(-1,0)
    }

    match ids.get_parent_id(4) {
        Some(id) => assert_eq!(id,1),
        None => assert_eq!(-1,0)
    }

    assert_eq!(ids.get_child_ids(1).len(),3);

    assert_eq!(ids.get_parent_ids().len(),2);

    

}
    */

/*
fn fn_test_parent_child() {
    
    //Test new parent child
    let mut p : ParentChild= ParentChild{item_id:1,children:Vec::new()};
    println!("{p}");
    let c= p.add_item(3);
    println!("{c}");
    println!("{p}");
    let l = c.item_id;
    println!("{l}");
    let _ = &p.add_item(4);
    let _ = &p.add_item(5);
    let _ = &p.add_item(6);
    let _ = &p.add_item(7);
    let _ = &p.add_item(8);
    let _ = &p.add_item(9);

    let node_index = 0;
    let c = &p.children[node_index];
    let l = c.children.len();
    let iid = c.item_id;
    println!("child count for node {node_index} with id {iid} is {l}");

    let node_index = 4;
    let c = &mut p.children[node_index];
    let l = c.children.len();
    let iid = c.item_id;
    println!("child count for node {node_index} with id {iid} is {l}");
    c.add_item(100);

    let l = c.children.len();
    let iid = c.item_id;
    println!("child count for node {node_index} with id {iid} is {l}");

    let subs = &p.get_sub_item_ids();
    let l = subs.len();
    println!("We have {l} sub items");
    for x in subs {
        println!("{x}");
    }

    let c = &mut p.children[node_index];
    let iid = c.item_id;
    
    let subs = &c.get_sub_item_ids();
    let l = subs.len();
    println!("We have {l} sub items for item with id{iid}");
    for x in subs {
        println!("{x}");
    }
    let s = String::new();
    p.print( s);
}
     */