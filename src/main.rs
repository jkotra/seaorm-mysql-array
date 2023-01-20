use sea_orm::*;
use sea_orm::{Database, DbErr};

mod entities;
use entities::employee;
use entities::prelude::*;
use entities::projects;

const DATABASE_URL: &str = "mysql://root:password@localhost:3306/emp_db";
const HELP: &str = "PROGRAM [show|add|rm {id}|clean]";

#[derive(Debug, Default)]
struct EmployeeModel {
    id: i64,
    name: String,
    projects: Vec<String>,
}

async fn get_db() -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(DATABASE_URL).await?;
    Ok(db)
}

async fn insert(emp: EmployeeModel) -> Result<(), DbErr> {
    let db = get_db().await?;

    let e = employee::ActiveModel {
        name: ActiveValue::Set(emp.name.clone()),
        ..Default::default()
    };

    let ires = Employee::insert(e).exec(&db).await?;
    println!(
        "employee {} inserted with id = {}",
        emp.name, ires.last_insert_id
    );

    for p in 0..emp.projects.len() {
        let proj_name = emp.projects.clone().get(p).unwrap().to_string();
        if proj_name.len() < 1 {
            continue;
        };
        let i_proj = projects::ActiveModel {
            emp_id: ActiveValue::Set(ires.last_insert_id),
            seq: ActiveValue::Set(p as i32),
            value: ActiveValue::Set(proj_name.clone()),
            ..Default::default()
        };
        let inserted = Projects::insert(i_proj).exec(&db).await.unwrap();
        println!(
            "project {} inserted with id = {}",
            proj_name, inserted.last_insert_id
        );
    }

    Ok(())
}

async fn add_emp() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = std::io::stdin();

    let mut obj = EmployeeModel {
        ..Default::default()
    };
    let mut in_projects = String::new();

    println!("Employee Name:");
    stdin.read_line(&mut obj.name)?;
    obj.name = obj.name.trim().to_string();

    println!("Employee Projects [seperated by comma(,)]:");
    stdin.read_line(&mut in_projects)?;
    in_projects = in_projects.trim().to_string();

    let mut proj_iter = in_projects.split(",").into_iter();

    loop {
        match proj_iter.next() {
            Some(x) => obj.projects.push(x.to_string()),
            None => break,
        }
    }

    println!("constructed object = {:?}", obj);

    insert(obj).await.unwrap();

    Ok(())
}

async fn remove_emp(id: i32) -> Result<(), DbErr> {
    println!("removing employee with id = {}", id);
    let db = get_db().await?;
    Projects::delete_many()
        .filter(projects::Column::EmpId.eq(id))
        .exec(&db)
        .await?;

    Employee::delete_by_id(id).exec(&db).await?;
    Ok(())
}

async fn remove_all() -> Result<(), DbErr> {
    let db = get_db().await?;
    for emp in Employee::find().all(&db).await? {
        remove_emp(emp.id).await?;
    }
    Ok(())
}

async fn find_all() -> Result<(), DbErr> {
    let db = get_db().await?;
    let mut data: Vec<EmployeeModel> = Vec::new();

    let all_emps = Employee::find().all(&db).await?;

    for emp in all_emps {
        let mut obj = EmployeeModel {
            ..Default::default()
        };
        obj.name = emp.name;
        obj.id = emp.id as i64;

        // get projects
        let emp_projs = Projects::find()
            .filter(projects::Column::EmpId.eq(emp.id))
            .all(&db)
            .await?;

        if emp_projs.len() > 1 {
            obj.projects = Vec::new();

            for proj in emp_projs {
                obj.projects.push(proj.value);
            }
        };

        data.push(obj);
    }

    println!("=== ALL DATA length={} ===", data.len());
    for empm in data {
        println!("{:?}", empm);
    }
    println!("=== END ALL DATA ===");

    Ok(())
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        let asked_of = args.get(1).unwrap();

        if asked_of.eq("show") {
            find_all().await.unwrap()
        } else if asked_of.eq("add") {
            add_emp().await.unwrap();
            find_all().await.unwrap();
        } else if asked_of.eq("rm") {
            let id = args.get(2).unwrap().parse::<i32>().unwrap();
            find_all().await.unwrap();
            remove_emp(id).await.unwrap();
        } else if asked_of.eq("clean") {
            remove_all().await.unwrap();
            find_all().await.unwrap();
        } else {
            println!("Err! {}", HELP);
        }
    } else {
        println!("Err! {}", HELP);
    };
}
