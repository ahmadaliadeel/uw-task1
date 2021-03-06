use std::{collections::hash_set::HashSet, hash::Hash};

#[derive(Debug, Hash, PartialEq, Eq)]
enum Value
{
  Num(u32),
  Any,
}


fn contains(list : &HashSet<Value>, value : u32) -> bool
{
  let mut include_all = false;

  if list.contains(&Value::Any)
  {
    return true;
  }
  else
  {
    let numbers : HashSet<u32> = list
      .iter()
      .filter_map(|x| match x
      {
        Value::Num(n) => Some(*n),
        Value::Any =>
        {
          include_all = true;
          None
        }
      })
      .collect();
    include_all || numbers.contains(&value)
  }
}


fn attempt(available2 : &Vec<u32>, allowed : &HashSet<Value>, prefered : &mut Vec<Value>) -> HashSet<u32>
{
  let available = available2
    .iter()
    .filter(|x| contains(&allowed, **x))
    .map(|x| *x)
    .collect::<Vec<_>>();

  let mut results = HashSet::new();
  let include_anyway = prefered
    .into_iter()
    .map(|x| match x
    {
      Value::Num(_) => false,
      Value::Any => true,
    })
    .reduce(|accum, item| accum || item)
    .unwrap();

  let allowed = available.clone();
  if include_anyway
  {
    return available.into_iter().collect::<HashSet<_>>();
  }
  for y in prefered
  {
    let mut avail = available.clone();
    let z = match avail.pop()
    {
      Some(x) => match y
      {
        Value::Num(z) =>
        {
          if allowed.contains(&z)
          {
            Some(*z)
          }
          else
          {
            let mut pos = None;
            for ax in available2
            {
              if x < *ax
              {
                break;
              }
              pos = Some(x);
            }
            pos
          }
        }
        Value::Any => None,
      },
      None => break,
    };
    if z.is_some()
    {
      results.insert(z.unwrap());
    }
  }
  results
}


#[cfg(test)]
mod tests
{
  use std::collections::HashSet;

  use crate::{Value, attempt};

  #[test]
  fn case_x_1()
  {
    let left : HashSet<u32> = vec![720].into_iter().collect();
    let available : Vec<u32> = vec![240, 360, 720].into_iter().collect();
    let allowed : HashSet<Value> = vec![Value::Num(360), Value::Num(720)].into_iter().collect();
    let mut prefered : Vec<Value> = vec![Value::Num(1080)].into_iter().collect();
    let right = attempt(&available, &allowed, &mut prefered);
    println!("x1 {:?}", right);
    assert!(left == right);
  }
  #[test]
  fn case_x_2()
  {
    let left : HashSet<u32> = vec![720].into_iter().collect();
    let available : Vec<u32> = vec![240, 720].into_iter().collect();
    let allowed : HashSet<Value> = vec![Value::Num(360), Value::Num(720)].into_iter().collect();
    let mut prefered : Vec<Value> = vec![Value::Num(1080)].into_iter().collect();
    let right = attempt(&available, &allowed, &mut prefered);
    println!("x2 {:?}", right);
    assert!(left == right);
  }
  #[test]
  fn case_x_3()
  {
    let left : HashSet<u32> = vec![].into_iter().collect();
    let available : Vec<u32> = vec![240].into_iter().collect();
    let allowed : HashSet<Value> = vec![Value::Num(360), Value::Num(720)].into_iter().collect();
    let mut prefered : Vec<Value> = vec![Value::Num(1080)].into_iter().collect();
    let right = attempt(&available, &allowed, &mut prefered);
    assert_eq!(left, right);
  }
  #[test]
  fn case_x_4()
  {
    let left : HashSet<u32> = vec![240, 360].into_iter().collect();
    let available : Vec<u32> = vec![240, 360, 720].into_iter().collect();
    let allowed : HashSet<Value> = vec![Value::Num(240), Value::Num(360), Value::Num(720), Value::Num(1080)]
      .into_iter()
      .collect();
    let mut prefered : Vec<Value> = vec![Value::Num(240), Value::Num(360)].into_iter().collect();
    let right = attempt(&available, &allowed, &mut prefered);
    assert_eq!(left, right);
  }
  #[test]
  fn case_x_5()
  {
    let left : HashSet<u32> = vec![240, 720].into_iter().collect();
    let available : Vec<u32> = vec![240, 720].into_iter().collect();
    let allowed : HashSet<Value> = vec![Value::Num(240), Value::Num(360), Value::Num(720), Value::Num(1080)]
      .into_iter()
      .collect();
    let mut prefered : Vec<Value> = vec![Value::Num(240), Value::Num(360)].into_iter().collect();
    let right = attempt(&available, &allowed, &mut prefered);
    assert_eq!(left, right);
  }
  #[test]
  fn case_x_6()
  {
    let left : HashSet<u32> = vec![240].into_iter().collect();
    let available : Vec<u32> = vec![240, 720].into_iter().collect();
    let allowed : HashSet<Value> = vec![Value::Num(240), Value::Num(360), Value::Num(1080)].into_iter().collect();
    let mut prefered : Vec<Value> = vec![Value::Num(240), Value::Num(360)].into_iter().collect();
    let right = attempt(&available, &allowed, &mut prefered);
    assert_eq!(left, right);
  }
  #[test]
  fn case_x_7()
  {
    let left : HashSet<u32> = vec![].into_iter().collect();
    let available : Vec<u32> = vec![720].into_iter().collect();
    let allowed : HashSet<Value> = vec![Value::Num(240), Value::Num(360), Value::Num(1080)].into_iter().collect();
    let mut prefered : Vec<Value> = vec![Value::Num(240), Value::Num(360)].into_iter().collect();
    let right = attempt(&available, &allowed, &mut prefered);
    assert_eq!(left, right);
  }
  #[test]
  fn case_x_8()
  {
    let left : HashSet<u32> = vec![360].into_iter().collect();
    let available : Vec<u32> = vec![240, 360].into_iter().collect();
    let allowed : HashSet<Value> = vec![Value::Num(240), Value::Num(360)].into_iter().collect();
    let mut prefered : Vec<Value> = vec![Value::Num(720), Value::Num(1080)].into_iter().collect();
    let right = attempt(&available, &allowed, &mut prefered);
    assert_eq!(left, right);
  }

  // Special Value
  #[test]
  fn case_xs_1()
  {
    let left : HashSet<u32> = vec![360, 720].into_iter().collect();
    let available : Vec<u32> = vec![240, 360, 720].into_iter().collect();
    let allowed : HashSet<Value> = vec![Value::Num(360), Value::Any].into_iter().collect();
    let mut prefered : Vec<Value> = vec![Value::Num(360), Value::Num(720)].into_iter().collect();
    let right = attempt(&available, &allowed, &mut prefered);
    assert_eq!(left, right);
  }
  #[test]
  fn case_xs_2()
  {
    let left : HashSet<u32> = vec![240, 360, 720].into_iter().collect();
    let available : Vec<u32> = vec![240, 360, 720].into_iter().collect();
    let allowed : HashSet<Value> = vec![Value::Num(240), Value::Num(360), Value::Any].into_iter().collect();
    let mut prefered : Vec<Value> = vec![Value::Any, Value::Num(720)].into_iter().collect();
    let right = attempt(&available, &allowed, &mut prefered);
    assert_eq!(left, right);
  }
  #[test]
  fn case_xs_3()
  {
    let left : HashSet<u32> = vec![360].into_iter().collect();
    let available : Vec<u32> = vec![240, 360, 720].into_iter().collect();
    let allowed : HashSet<Value> = vec![Value::Num(360), Value::Num(1080)].into_iter().collect();
    let mut prefered : Vec<Value> = vec![Value::Any, Value::Num(720)].into_iter().collect();
    let right = attempt(&available, &allowed, &mut prefered);
    assert_eq!(left, right);
  }
  #[test]
  fn case_xs_4()
  {
    let left : HashSet<u32> = vec![].into_iter().collect();
    let available : Vec<u32> = vec![240, 360, 720].into_iter().collect();
    let allowed : HashSet<Value> = vec![Value::Num(1080)].into_iter().collect();
    let mut prefered : Vec<Value> = vec![Value::Any, Value::Num(720)].into_iter().collect();
    let right = attempt(&available, &allowed, &mut prefered);
    assert_eq!(left, right);
  }
}
