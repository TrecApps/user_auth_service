use std::collections::HashMap;

pub fn split_string_n(string: &String, splitter: &str, size: usize) -> Vec<String>
{
    let mut ret = Vec::<String>::new();

    if string.find(splitter).is_none()
    {
        
        ret.push(string.to_string());
    }
    else
    {
        for section in string.splitn(size, splitter)
        {
            ret.push(String::from(section));
        }
    }

    ret
}

pub fn split_string(string: &String, splitter: &str) -> Vec<String>
{
    let mut ret = Vec::<String>::new();

    if string.find(splitter).is_none()
    {
        ret.push(string.to_string());
    }
    else
    {
        for section in string.split(splitter)
        {
            ret.push(String::from(section));
        }
    }
    ret
}

pub fn get_map_from_string(contents: &String, entry_split: &str, key_value_split: &str) -> Option<HashMap<String, String>>
{
    let mut ret = HashMap::<String, String>::new();

    if contents.len() == 0
    {
        return Some(ret);
    }

    for entry in contents.split(entry_split)
    {
        let real_entry = String::from(entry);

        let pair = split_string_n(&real_entry, key_value_split, 2);

        if pair.len() != 2
        {
           

            continue;
        }
        

        let (key, value) = (pair.get(0).expect("msg: &str"), pair.get(1).expect("msg: &str"));
        ret.insert(key.clone(),value.clone());
    }


    Some(ret)
}