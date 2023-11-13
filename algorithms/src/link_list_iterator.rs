use algorithms_utils::link_list::*;

fn link_list_iterator<T: Copy>(list: Link<T>) -> Vec<T> {
    let mut result = Vec::new();
    let mut cur_node = list;
    loop {
        if let Some(node) = cur_node.clone() {
            result.push(node.borrow().value.clone());
            cur_node = node.borrow().next.clone();
        } else {
            break;
        }
    }
    return result;
}

#[cfg(test)]
mod test {
    use super::*;
    use algorithms_utils::link_list;

    #[test]
    fn test_link_iterator() {
        let list = link_list!(2, link_list!(10, link_list!(108, None)));
        let list1 = list.clone();
        let result = link_list_iterator(list1);
        assert_eq!(result, vec![2, 10, 108]);
    }
}
