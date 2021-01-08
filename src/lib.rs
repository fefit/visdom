
use std::cell::RefCell;
use std::rc::Rc;
use rphtml::parser::{Attr, AttrData, Doc, Node, NodeType};
use ntree::selector::interface::{ INodeType, INodeTrait, IAttrValue, BoxDynNode, MaybeResult, Result, NodeList };
pub struct Visdom{
  node: Rc<RefCell<Node>>
}

impl INodeTrait for Visdom {
  /// impl `cloned`
  fn cloned(&self) -> Box<dyn INodeTrait> {
    Box::new(Visdom{
      node: self.node.clone()
    })
  }
  /// impl `tag_name`
  fn tag_name(&self) -> &str {
    if let Some(meta) = self.node.borrow().meta{
      return meta.borrow().get_name(false).as_str();
    }
    panic!("Wrong `tag_name`")
  }
  /// impl `node_type`
  fn node_type(&self) -> INodeType {
    match self.node.borrow().node_type{
      NodeType::AbstractRoot => INodeType::Document,
      NodeType::Comment => INodeType::Comment,
      NodeType::Text => INodeType::Text,
      NodeType::SpacesBetweenTag => INodeType::Spaces,
      NodeType::Tag => INodeType::Element,
      _ => INodeType::Other
    }
  }
  /// impl `parent`
  fn parent<'b>(&self) -> MaybeResult<'b> {
    if let Some(parent) = self.node.borrow().parent{
      if let Some(node) = parent.upgrade(){
        let cur = Visdom{
          node 
        };
        return Ok(Some(Box::new(cur)));
      }else{
        return Err("The parent is destoryed.");
      }
    }
    Ok(None)
  }
  /// impl `children`
  fn children<'b>(&self) -> Result<'b> {
    if let Some(childs) = self.node.borrow().childs{
      let mut result = NodeList::with_capacity(childs.len());
      for cur in childs{
        if cur.borrow().node_type == NodeType::Tag{
          result.get_mut_ref().push(Box::new(Visdom{
            node: cur.clone()
          }));
        }
      }
      return Ok(result);
    }
    Ok(NodeList::new())
  }
  /// impl `get_attribute`
  fn get_attribute(&self, name: &str) -> Option<IAttrValue> {
     if let Some(meta) = self.node.borrow().meta{
       for attr in &meta.borrow().attrs{
        if let Some(key) = &attr.key{
          if key.content == name{
            if let Some(value) = &attr.value{
              let attr_value = value.content.clone();
              return Some(IAttrValue::Value(attr_value, attr.quote.clone()));
            } else {
              return Some(IAttrValue::True);
            }
          }
        }
      }
     } 
     None
  }
  /// impl `set_attribute`
  fn set_attribute(&mut self, name: &str, value: Option<&str>) {
    let mut need_quote = false;
    let mut quote:char = '"';
    if let Some(meta) = self.node.borrow().meta{     
      let value = value.map(|v|{
        let mut find_quote: bool = false;
        let mut result = String::with_capacity(v.len());
        // loop the chars
        for ch in v.chars(){
          if !need_quote{
            need_quote = Attr::need_quoted_char(&ch);
          }
          if ch == '"' || ch == '\''{
            if find_quote{
              if quote == ch{
                // find more quotes
                result.push('\\');
              }
            }else{
              // if first is double quote, change the variable `quote` to single quote 
              find_quote = true;
              if ch == '"'{
                quote = '\'';
              }
            }
          }
          result.push(ch);
        }
      });
      for attr in &mut meta.borrow_mut().attrs{
        if let Some(key) = &attr.key{
          if key.content == name{
            
            return;
          }
        }
      }
    }
    panic!("Can't apply 'set_attribute' to node");
  }
}
impl Visdom{

}