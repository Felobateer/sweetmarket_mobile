use std::sync::Arc;

pub fn rate_routes(rates: Arc<Rates>) -> impl Fn(&str, Option<&str>, Option<&Rate>) -> Result<String, String> {
    move |action, id, body| match action {
        "get_all" => match rates.get_all() {
            Ok(rate_list) => Ok(format!("{:?}", rate_list)),
            Err(err) => Err(err),
        },
        "get_by_id" => {
            if let Some(id) = id.and_then(|id| id.parse().ok()) {
                match rates.get_by_id(&id) {
                    Ok(rate) => Ok(format!("{:?}", rate)),
                    Err(err) => Err(err),
                }
            } else {
                Err("Invalid ID".into())
            }
        }
        "add" => {
            if let Some(rate) = body {
                rates.add(rate.clone()).map(|_| "Rate added successfully".into())
            } else {
                Err("Invalid body".into())
            }
        }
        "edit" => {
            if let Some(rate) = body {
                rates.edit(rate.clone()).map(|_| "Rate updated successfully".into())
            } else {
                Err("Invalid body".into())
            }
        }
        "delete" => {
            if let Some(id) = id.and_then(|id| id.parse().ok()) {
                rates.delete(&id).map(|_| "Rate deleted successfully".into())
            } else {
                Err("Invalid ID".into())
            }
        }
        _ => Err("Invalid action".into()),
    }
}
