
pub trait Ast{
    fn to_json(&self, is_pretty: bool) -> String;
}