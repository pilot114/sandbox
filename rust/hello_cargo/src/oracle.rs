use oracle::{Connection};

fn main() {

    let con_string = "(DESCRIPTION =(ADDRESS_LIST =(ADDRESS = (PROTOCOL = TCP)(HOST = 192.168.0.160)(PORT = 1521)))(CONNECT_DATA =(SID = feo)))";
    let conn = Connection::connect("tech_corp_portal[test]", "hebrgd635svrg", con_string);

    let sql = "select * from test.cp_emp where id = 4026";

    for info in conn.unwrap().query(sql, &[]).unwrap().column_info() {
        print!(" {:14}|", info.name());
        print!(" {:14}|", info.oracle_type().to_string());
    }
}
