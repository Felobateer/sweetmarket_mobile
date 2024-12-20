use std::sync::Arc;

pub fn payment_routes(payments: Arc<Payments>) -> impl Fn(&str, Option<&str>, Option<&Payment>) -> Result<String, String> {
    move |action, id, body| match action {
        "get_all" => match payments.get_all() {
            Ok(payment_list) => Ok(format!("{:?}", payment_list)),
            Err(err) => Err(err),
        },
        "get_by_id" => {
            if let Some(id) = id.and_then(|id| id.parse().ok()) {
                match payments.get_by_id(&id) {
                    Ok(payment) => Ok(format!("{:?}", payment)),
                    Err(err) => Err(err),
                }
            } else {
                Err("Invalid ID".into())
            }
        }
        "add" => {
            if let Some(payment) = body {
                payments.add(payment.clone()).map(|_| "Payment added successfully".into())
            } else {
                Err("Invalid body".into())
            }
        }
        "edit" => {
            if let Some(payment) = body {
                payments.edit(payment.clone()).map(|_| "Payment updated successfully".into())
            } else {
                Err("Invalid body".into())
            }
        }
        "delete" => {
            if let Some(id) = id.and_then(|id| id.parse().ok()) {
                payments.delete(&id).map(|_| "Payment deleted successfully".into())
            } else {
                Err("Invalid ID".into())
            }
        }
        _ => Err("Invalid action".into()),
    }
}
