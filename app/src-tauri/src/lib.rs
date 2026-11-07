use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf, sync::atomic::{AtomicU64, Ordering}};
use tauri::{Manager, State};

#[derive(Clone)]
struct AppState {
  database_path: PathBuf,
}
static ID_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct Organization {
  id: String,
  name: String,
  ceo_display_name: String,
  status: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct Role {
  id: String,
  organization_id: String,
  title: String,
  description: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct RuntimeBinding {
  id: String,
  provider_kind: String,
  display_name: String,
  status: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct Employee {
  id: String,
  organization_id: String,
  display_name: String,
  role_id: String,
  role_title: String,
  manager_employee_id: Option<String>,
  runtime_binding_id: Option<String>,
  runtime_display_name: Option<String>,
  status: String,
  capabilities: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateEmployeeInput {
  display_name: String,
  role_id: String,
  manager_employee_id: Option<String>,
  runtime_binding_id: Option<String>,
  status: String,
  capabilities: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateEmployeeInput {
  id: String,
  display_name: String,
  role_id: String,
  manager_employee_id: Option<String>,
  runtime_binding_id: Option<String>,
  status: String,
  capabilities: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OrganizationInput { name:String, ceo_display_name:String }
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RoleInput { title:String, description:String }
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct EmployeeProfileEvent { id:String, employee_id:String, occurred_at:String, actor_kind:String, source_kind:String, event_type:String, summary:String }

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct AuthorityDecision { allowed: bool, reason_code: String, reason: String, policy_id: Option<String>, policy_source: Option<String>, matched_limits: Option<String> }
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EvaluatePolicyInput { employee_id:String, work_scope:String, action:String, resource_scope:String, requested_quota:Option<f64>, requires_delegate:bool, requires_self_approve:bool }
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct AuthorityPolicy { id:String, employee_id:Option<String>, role_id:Option<String>, work_scope:String, action:String, resource_scope:String, expires_at:Option<String>, quota:Option<f64>, can_delegate:bool, can_self_approve:bool, status:String, supersedes_id:Option<String>, reason:String }
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateAuthorityPolicyInput { employee_id:Option<String>, role_id:Option<String>, work_scope:String, action:String, resource_scope:String, expires_at:Option<String>, quota:Option<f64>, can_delegate:bool, can_self_approve:bool, reason:String }
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RevokeAuthorityPolicyInput { id:String, reason:String }
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct AuthorityPolicyEvent { id:String, policy_id:String, occurred_at:String, actor_kind:String, source_kind:String, event_type:String, reason:String, related_policy_id:Option<String> }

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct LedgerEvent { id: String, occurred_at: String, recorded_at: String, event_type: String, actor_kind: String, actor_employee_id: Option<String>, source_kind: String, summary: String, details_json: String, correction_of_event_id: Option<String> }
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AppendLedgerInput { event_type: String, actor_kind: String, actor_employee_id: Option<String>, source_kind: String, summary: String, details_json: String, correction_of_event_id: Option<String> }
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct Report { id: String, title: String, status: String, work_done: String, result: String, uncertainties: String, delegation_summary: String, next_steps: String, created_at: String }
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateReportInput { title: String, work_done: String, result: String, uncertainties: String, delegation_summary: String, next_steps: String, evidence_location: Option<String> }

fn database_connection(path: &PathBuf) -> Result<Connection, String> {
  let mut connection = Connection::open(path).map_err(|error| error.to_string())?;
  connection.execute_batch(
    "PRAGMA foreign_keys = ON;
     CREATE TABLE IF NOT EXISTS organizations (
       id TEXT PRIMARY KEY, name TEXT NOT NULL, ceo_display_name TEXT NOT NULL, status TEXT NOT NULL
     );
     CREATE TABLE IF NOT EXISTS roles (
       id TEXT PRIMARY KEY, organization_id TEXT NOT NULL REFERENCES organizations(id), title TEXT NOT NULL, description TEXT NOT NULL
     );
     CREATE TABLE IF NOT EXISTS runtime_bindings (
       id TEXT PRIMARY KEY, provider_kind TEXT NOT NULL, display_name TEXT NOT NULL, connection_reference TEXT NOT NULL, status TEXT NOT NULL
     );
     CREATE TABLE IF NOT EXISTS employees (
       id TEXT PRIMARY KEY, organization_id TEXT NOT NULL REFERENCES organizations(id), display_name TEXT NOT NULL,
       role_id TEXT NOT NULL REFERENCES roles(id), manager_employee_id TEXT REFERENCES employees(id),
       runtime_binding_id TEXT REFERENCES runtime_bindings(id), status TEXT NOT NULL, capabilities TEXT NOT NULL DEFAULT '',
       CHECK(status IN ('active', 'paused', 'inactive'))
     );
     CREATE TABLE IF NOT EXISTS authority_envelopes (
       id TEXT PRIMARY KEY, employee_id TEXT NOT NULL REFERENCES employees(id), action TEXT NOT NULL,
       resource_scope TEXT NOT NULL, work_scope TEXT NOT NULL, expires_at TEXT, budget_limit REAL,
       delegation_mode TEXT NOT NULL, self_approval INTEGER NOT NULL CHECK(self_approval IN (0, 1)),
       status TEXT NOT NULL CHECK(status IN ('active', 'revoked'))
     );
     CREATE TABLE IF NOT EXISTS ledger_events (
       id TEXT PRIMARY KEY, occurred_at TEXT NOT NULL, recorded_at TEXT NOT NULL, event_type TEXT NOT NULL,
       actor_kind TEXT NOT NULL, actor_employee_id TEXT REFERENCES employees(id), source_kind TEXT NOT NULL,
       summary TEXT NOT NULL, details_json TEXT NOT NULL, correction_of_event_id TEXT REFERENCES ledger_events(id)
     );
     CREATE TRIGGER IF NOT EXISTS ledger_events_no_update BEFORE UPDATE ON ledger_events BEGIN SELECT RAISE(ABORT, 'ledger events are append-only'); END;
     CREATE TRIGGER IF NOT EXISTS ledger_events_no_delete BEFORE DELETE ON ledger_events BEGIN SELECT RAISE(ABORT, 'ledger events are append-only'); END;
     CREATE TRIGGER IF NOT EXISTS runtime_bindings_no_update BEFORE UPDATE ON runtime_bindings BEGIN SELECT RAISE(ABORT, 'runtime bindings are immutable in J-01'); END;
     CREATE TRIGGER IF NOT EXISTS runtime_bindings_no_delete BEFORE DELETE ON runtime_bindings BEGIN SELECT RAISE(ABORT, 'runtime bindings are immutable in J-01'); END;
     CREATE TABLE IF NOT EXISTS employee_profile_events (
       id TEXT PRIMARY KEY, employee_id TEXT NOT NULL REFERENCES employees(id), occurred_at TEXT NOT NULL,
       actor_kind TEXT NOT NULL, source_kind TEXT NOT NULL, event_type TEXT NOT NULL, summary TEXT NOT NULL
     );
     CREATE TRIGGER IF NOT EXISTS employee_profile_events_no_update BEFORE UPDATE ON employee_profile_events BEGIN SELECT RAISE(ABORT, 'employee profile events are append-only'); END;
     CREATE TRIGGER IF NOT EXISTS employee_profile_events_no_delete BEFORE DELETE ON employee_profile_events BEGIN SELECT RAISE(ABORT, 'employee profile events are append-only'); END;
     CREATE TABLE IF NOT EXISTS reports (
       id TEXT PRIMARY KEY, title TEXT NOT NULL, status TEXT NOT NULL, work_done TEXT NOT NULL, result TEXT NOT NULL,
       uncertainties TEXT NOT NULL, delegation_summary TEXT NOT NULL, next_steps TEXT NOT NULL, created_at TEXT NOT NULL
     );
     CREATE TABLE IF NOT EXISTS artifacts (
       id TEXT PRIMARY KEY, report_id TEXT NOT NULL REFERENCES reports(id), location TEXT NOT NULL, integrity_hint TEXT NOT NULL, created_at TEXT NOT NULL
     );
     CREATE TABLE IF NOT EXISTS authority_policy_versions (
       id TEXT PRIMARY KEY, employee_id TEXT REFERENCES employees(id), role_id TEXT REFERENCES roles(id), work_scope TEXT NOT NULL, action TEXT NOT NULL, resource_scope TEXT NOT NULL,
       expires_at TEXT, quota REAL, can_delegate INTEGER NOT NULL, can_self_approve INTEGER NOT NULL, status TEXT NOT NULL, supersedes_id TEXT REFERENCES authority_policy_versions(id), reason TEXT NOT NULL,
       CHECK((employee_id IS NOT NULL AND role_id IS NULL) OR (employee_id IS NULL AND role_id IS NOT NULL)), CHECK(status IN ('active','narrowed','revoked'))
     );
     CREATE TRIGGER IF NOT EXISTS authority_policy_exactly_one_subject_insert BEFORE INSERT ON authority_policy_versions
     WHEN (NEW.employee_id IS NULL AND NEW.role_id IS NULL) OR (NEW.employee_id IS NOT NULL AND NEW.role_id IS NOT NULL)
     BEGIN SELECT RAISE(ABORT, 'authority policy must have exactly one subject'); END;
     CREATE TRIGGER IF NOT EXISTS authority_policy_exactly_one_subject_update BEFORE UPDATE OF employee_id, role_id ON authority_policy_versions
     WHEN (NEW.employee_id IS NULL AND NEW.role_id IS NULL) OR (NEW.employee_id IS NOT NULL AND NEW.role_id IS NOT NULL)
     BEGIN SELECT RAISE(ABORT, 'authority policy must have exactly one subject'); END;
     CREATE TABLE IF NOT EXISTS authority_policy_events (
       id TEXT PRIMARY KEY, policy_id TEXT NOT NULL REFERENCES authority_policy_versions(id), occurred_at TEXT NOT NULL,
       actor_kind TEXT NOT NULL, source_kind TEXT NOT NULL, event_type TEXT NOT NULL, reason TEXT NOT NULL,
       related_policy_id TEXT REFERENCES authority_policy_versions(id)
     );
     CREATE TRIGGER IF NOT EXISTS authority_policy_events_no_update BEFORE UPDATE ON authority_policy_events BEGIN SELECT RAISE(ABORT, 'authority policy events are append-only'); END;
     CREATE TRIGGER IF NOT EXISTS authority_policy_events_no_delete BEFORE DELETE ON authority_policy_events BEGIN SELECT RAISE(ABORT, 'authority policy events are append-only'); END;",
  ).map_err(|error| error.to_string())?;
  ensure_employee_capabilities_column(&connection)?;
  migrate_legacy_authority_envelopes(&mut connection)?;
  validate_existing_policy_subjects(&connection)?;
  Ok(connection)
}

fn now() -> String { chrono::Utc::now().to_rfc3339() }
fn ensure_employee_capabilities_column(connection:&Connection)->Result<(),String>{
  let mut statement=connection.prepare("PRAGMA table_info(employees)").map_err(|e|e.to_string())?;
  let columns=statement.query_map([],|row|row.get::<_,String>(1)).map_err(|e|e.to_string())?.collect::<Result<Vec<_>,_>>().map_err(|e|e.to_string())?;
  if !columns.iter().any(|column|column=="capabilities") { connection.execute("ALTER TABLE employees ADD COLUMN capabilities TEXT NOT NULL DEFAULT ''",[]).map_err(|e|e.to_string())?; }
  Ok(())
}
fn ledger_from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<LedgerEvent> { Ok(LedgerEvent { id: row.get(0)?, occurred_at: row.get(1)?, recorded_at: row.get(2)?, event_type: row.get(3)?, actor_kind: row.get(4)?, actor_employee_id: row.get(5)?, source_kind: row.get(6)?, summary: row.get(7)?, details_json: row.get(8)?, correction_of_event_id: row.get(9)? }) }
fn append_event(connection: &Connection, input: AppendLedgerInput) -> Result<LedgerEvent, String> {
  if input.event_type.trim().is_empty() || input.summary.trim().is_empty() || input.details_json.trim().is_empty() { return Err("Ledger event cần loại, tóm tắt và chi tiết.".to_string()); }
  if input.event_type == "ledger.correction_recorded" && input.correction_of_event_id.is_none() { return Err("Đính chính phải liên kết event gốc.".to_string()); }
  let id = format!("led_{}", uuid_suffix()); let timestamp = now();
  connection.execute("INSERT INTO ledger_events VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)", params![id, timestamp, timestamp, input.event_type, input.actor_kind, input.actor_employee_id, input.source_kind, input.summary, input.details_json, input.correction_of_event_id]).map_err(|e| e.to_string())?;
  connection.query_row("SELECT id, occurred_at, recorded_at, event_type, actor_kind, actor_employee_id, source_kind, summary, details_json, correction_of_event_id FROM ledger_events WHERE id=?1", [id], ledger_from_row).map_err(|e| e.to_string())
}

fn policy_from_row(r:&rusqlite::Row<'_>)->rusqlite::Result<AuthorityPolicy>{Ok(AuthorityPolicy{id:r.get(0)?,employee_id:r.get(1)?,role_id:r.get(2)?,work_scope:r.get(3)?,action:r.get(4)?,resource_scope:r.get(5)?,expires_at:r.get(6)?,quota:r.get(7)?,can_delegate:r.get(8)?,can_self_approve:r.get(9)?,status:r.get(10)?,supersedes_id:r.get(11)?,reason:r.get(12)?})}
fn policy_event_from_row(r:&rusqlite::Row<'_>)->rusqlite::Result<AuthorityPolicyEvent>{Ok(AuthorityPolicyEvent{id:r.get(0)?,policy_id:r.get(1)?,occurred_at:r.get(2)?,actor_kind:r.get(3)?,source_kind:r.get(4)?,event_type:r.get(5)?,reason:r.get(6)?,related_policy_id:r.get(7)?})}
fn migrate_legacy_authority_envelopes(connection:&mut Connection)->Result<(),String>{
  let mut s=connection.prepare("SELECT id,employee_id,action,resource_scope,work_scope,expires_at,budget_limit,delegation_mode,self_approval,status FROM authority_envelopes").map_err(|e|e.to_string())?;
  let legacy=s.query_map([],|r|Ok((r.get::<_,String>(0)?,r.get::<_,String>(1)?,r.get::<_,String>(2)?,r.get::<_,String>(3)?,r.get::<_,String>(4)?,r.get::<_,Option<String>>(5)?,r.get::<_,Option<f64>>(6)?,r.get::<_,String>(7)?,r.get::<_,bool>(8)?,r.get::<_,String>(9)?))).map_err(|e|e.to_string())?.collect::<Result<Vec<_>,_>>().map_err(|e|e.to_string())?;
  drop(s);
  for (legacy_id,employee_id,action,resource_scope,work_scope,expires_at,quota,delegation_mode,self_approval,status) in legacy {
    let id=format!("authp_migrated_{legacy_id}");
    let exists:bool=connection.query_row("SELECT EXISTS(SELECT 1 FROM authority_policy_versions WHERE id=?1)",[&id],|r|r.get(0)).map_err(|e|e.to_string())?;
    if exists { continue; }
    let mapped_status=if status=="active"{"active"}else{"revoked"};
    let reason=format!("Migrated from retired legacy envelope {legacy_id}");
    let tx=connection.transaction().map_err(|e|e.to_string())?;
    tx.execute("INSERT INTO authority_policy_versions VALUES(?1,?2,NULL,?3,?4,?5,?6,?7,?8,?9,?10,NULL,?11)",params![id,employee_id,work_scope,action,resource_scope,expires_at,quota,delegation_mode!="none",self_approval,mapped_status,reason]).map_err(|e|e.to_string())?;
    tx.execute("INSERT INTO authority_policy_events VALUES(?1,?2,?3,'system','migration','authority.migrated',?4,NULL)",params![format!("apol_evt_{}",uuid_suffix()),id,now(),format!("Migrated legacy envelope {legacy_id}")]).map_err(|e|e.to_string())?;
    tx.commit().map_err(|e|e.to_string())?;
  }
  Ok(())
}
fn validate_existing_policy_subjects(connection:&Connection)->Result<(),String>{
  let invalid:i64=connection.query_row("SELECT COUNT(*) FROM authority_policy_versions WHERE (employee_id IS NULL AND role_id IS NULL) OR (employee_id IS NOT NULL AND role_id IS NOT NULL)",[],|r|r.get(0)).map_err(|e|e.to_string())?;
  if invalid>0 { return Err("Dữ liệu policy cũ có subject mơ hồ; cần migration thủ công trước khi hệ thống đánh giá quyền.".into()); }
  Ok(())
}
fn scope_includes(parent:&str, child:&str)->bool { parent=="*" || parent==child || parent.strip_suffix("/*").map(|p| child.starts_with(p)&&child[p.len()..].starts_with('/')).unwrap_or(false) }
fn is_narrower(old:&AuthorityPolicy,input:&CreateAuthorityPolicyInput)->bool { old.employee_id==input.employee_id&&old.role_id==input.role_id&&scope_includes(&old.work_scope,&input.work_scope)&&scope_includes(&old.resource_scope,&input.resource_scope)&&old.action==input.action&&old.quota.map(|q|input.quota.map(|n|n<=q).unwrap_or(false)).unwrap_or(true)&&(!input.can_delegate||old.can_delegate)&&(!input.can_self_approve||old.can_self_approve)&&old.expires_at.as_ref().map(|e|input.expires_at.as_ref().map(|n|n<=e).unwrap_or(false)).unwrap_or(true) }
fn has_exactly_one_subject(input:&CreateAuthorityPolicyInput)->bool { input.employee_id.is_some() != input.role_id.is_some() }
fn validate_runtime_display_field(value:&str,field:&str)->Result<(),String>{let lower=value.trim().to_ascii_lowercase();if value.trim().is_empty(){return Err(format!("{field} không được để trống."))}if ["api_key","apikey","secret","token","password","bearer ","sk-","-----begin"].iter().any(|marker|lower.contains(marker)){return Err(format!("{field} không được chứa giá trị trông như bí mật/credential."))}Ok(())}
fn insert_seed_runtime_binding(connection:&Connection,id:&str,provider_kind:&str,display_name:&str,connection_reference:&str,status:&str)->Result<(),String>{
  validate_runtime_display_field(provider_kind,"Runtime provider")?;validate_runtime_display_field(display_name,"Runtime display name")?;
  connection.execute("INSERT INTO runtime_bindings VALUES(?1,?2,?3,?4,?5)",params![id,provider_kind,display_name,connection_reference,status]).map_err(|e|e.to_string())?;Ok(())
}

fn employee_from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Employee> {
  Ok(Employee {
    id: row.get(0)?, organization_id: row.get(1)?, display_name: row.get(2)?, role_id: row.get(3)?,
    role_title: row.get(4)?, manager_employee_id: row.get(5)?, runtime_binding_id: row.get(6)?,
    runtime_display_name: row.get(7)?, status: row.get(8)?, capabilities: row.get(9)?,
  })
}
fn profile_event_from_row(row:&rusqlite::Row<'_>)->rusqlite::Result<EmployeeProfileEvent>{Ok(EmployeeProfileEvent{id:row.get(0)?,employee_id:row.get(1)?,occurred_at:row.get(2)?,actor_kind:row.get(3)?,source_kind:row.get(4)?,event_type:row.get(5)?,summary:row.get(6)?})}
#[cfg(test)]
fn append_profile_event(connection:&Connection,employee_id:&str,event_type:&str,summary:&str)->Result<(),String>{connection.execute("INSERT INTO employee_profile_events VALUES(?1,?2,?3,'ceo','ceo_action',?4,?5)",params![format!("eprof_{}",uuid_suffix()),employee_id,now(),event_type,summary]).map_err(|e|e.to_string())?;Ok(())}

fn list_employees_from(connection: &Connection) -> Result<Vec<Employee>, String> {
  let mut statement = connection.prepare(
    "SELECT e.id, e.organization_id, e.display_name, e.role_id, r.title, e.manager_employee_id,
            e.runtime_binding_id, rb.display_name, e.status, e.capabilities
       FROM employees e JOIN roles r ON r.id = e.role_id
       LEFT JOIN runtime_bindings rb ON rb.id = e.runtime_binding_id
      ORDER BY e.display_name COLLATE NOCASE",
  ).map_err(|error| error.to_string())?;
  statement.query_map([], employee_from_row).map_err(|error| error.to_string())?
    .collect::<Result<Vec<_>, _>>().map_err(|error| error.to_string())
}

fn ensure_valid_manager(connection: &Connection, employee_id: Option<&str>, manager_id: Option<&str>) -> Result<(), String> {
  let Some(manager_id) = manager_id else { return Ok(()); };
  if employee_id == Some(manager_id) { return Err("Một nhân viên không thể tự báo cáo cho chính mình.".to_string()); }
  let mut cursor = Some(manager_id.to_string());
  while let Some(current) = cursor {
    if employee_id == Some(current.as_str()) { return Err("Tuyến báo cáo này tạo chu trình quản lý.".to_string()); }
    cursor = connection.query_row("SELECT manager_employee_id FROM employees WHERE id = ?1", [current], |row| row.get::<_, Option<String>>(0)).optional().map_err(|error| error.to_string())?.flatten();
  }
  Ok(())
}

fn ensure_sample_data(connection: &Connection) -> Result<(), String> {
  let exists: bool = connection.query_row(
    "SELECT EXISTS(SELECT 1 FROM organizations WHERE id = 'org_aicorp_hq')", [], |row| row.get(0),
  ).map_err(|error| error.to_string())?;
  if exists { return Ok(()); }
  connection.execute("INSERT INTO organizations VALUES (?1, ?2, ?3, ?4)", params!["org_aicorp_hq", "AI Corp", "CEO", "active"])
    .map_err(|error| error.to_string())?;
  connection.execute("INSERT INTO roles VALUES (?1, ?2, ?3, ?4)", params!["role_operations", "org_aicorp_hq", "Operations Agent", "Coordinates assigned company operations."])
    .map_err(|error| error.to_string())?;
  insert_seed_runtime_binding(connection,"rtb_local_preview","local","Local preview runtime","local://preview","available")?;
  connection.execute(
    "INSERT INTO employees (id,organization_id,display_name,role_id,manager_employee_id,runtime_binding_id,status,capabilities) VALUES (?1,?2,?3,?4,NULL,?5,?6,?7)",
    params!["emp_ops_001", "org_aicorp_hq", "An", "role_operations", "rtb_local_preview", "active", ""],
  ).map_err(|error| error.to_string())?;
  Ok(())
}

#[tauri::command]
fn initialize_demo_organization(state: State<'_, AppState>) -> Result<Vec<Employee>, String> {
  let connection = database_connection(&state.database_path)?;
  ensure_sample_data(&connection)?;
  list_employees_from(&connection)
}

fn current_organization_id(connection:&Connection)->Result<String,String>{connection.query_row("SELECT id FROM organizations LIMIT 1",[],|row|row.get(0)).map_err(|_|"Chưa có tổ chức local để thao tác.".to_string())}
#[tauri::command]
fn create_organization(input:OrganizationInput,state:State<'_,AppState>)->Result<Organization,String>{
  if input.name.trim().is_empty()||input.ceo_display_name.trim().is_empty(){return Err("Tổ chức cần tên và tên CEO.".into())}
  let c=database_connection(&state.database_path)?;let exists:bool=c.query_row("SELECT EXISTS(SELECT 1 FROM organizations)",[],|r|r.get(0)).map_err(|e|e.to_string())?;
  if exists{return Err("AI Corp local hiện chỉ hỗ trợ một tổ chức; hãy cập nhật tổ chức hiện hành.".into())}
  let id=format!("org_{}",uuid_suffix());c.execute("INSERT INTO organizations VALUES(?1,?2,?3,'active')",params![id,input.name.trim(),input.ceo_display_name.trim()]).map_err(|e|e.to_string())?;
  Ok(Organization{id,name:input.name.trim().into(),ceo_display_name:input.ceo_display_name.trim().into(),status:"active".into()})
}
#[tauri::command]
fn update_organization(input:OrganizationInput,state:State<'_,AppState>)->Result<Organization,String>{
  if input.name.trim().is_empty()||input.ceo_display_name.trim().is_empty(){return Err("Tổ chức cần tên và tên CEO.".into())}
  let c=database_connection(&state.database_path)?;let id=current_organization_id(&c)?;
  c.execute("UPDATE organizations SET name=?2,ceo_display_name=?3 WHERE id=?1",params![id,input.name.trim(),input.ceo_display_name.trim()]).map_err(|e|e.to_string())?;
  Ok(Organization{id,name:input.name.trim().into(),ceo_display_name:input.ceo_display_name.trim().into(),status:"active".into()})
}

#[tauri::command]
fn get_organization(state: State<'_, AppState>) -> Result<Organization, String> {
  let connection = database_connection(&state.database_path)?;
  connection.query_row(
    "SELECT id, name, ceo_display_name, status FROM organizations LIMIT 1", [],
    |row| Ok(Organization { id: row.get(0)?, name: row.get(1)?, ceo_display_name: row.get(2)?, status: row.get(3)? }),
  ).optional().map_err(|error| error.to_string())?
    .ok_or_else(|| "Chưa khởi tạo tổ chức mẫu.".to_string())
}

#[tauri::command]
fn list_roles(state: State<'_, AppState>) -> Result<Vec<Role>, String> {
  let connection = database_connection(&state.database_path)?;
  let mut statement = connection.prepare("SELECT id, organization_id, title, description FROM roles ORDER BY title")
    .map_err(|error| error.to_string())?;
  statement.query_map([], |row| Ok(Role { id: row.get(0)?, organization_id: row.get(1)?, title: row.get(2)?, description: row.get(3)? }))
    .map_err(|error| error.to_string())?.collect::<Result<Vec<_>, _>>().map_err(|error| error.to_string())
}
#[tauri::command]
fn create_role(input:RoleInput,state:State<'_,AppState>)->Result<Role,String>{
  if input.title.trim().is_empty(){return Err("Chức vụ cần tên.".into())}let c=database_connection(&state.database_path)?;let organization_id=current_organization_id(&c)?;let id=format!("role_{}",uuid_suffix());
  c.execute("INSERT INTO roles VALUES(?1,?2,?3,?4)",params![id,organization_id,input.title.trim(),input.description.trim()]).map_err(|e|e.to_string())?;
  Ok(Role{id,organization_id,title:input.title.trim().into(),description:input.description.trim().into()})
}
#[tauri::command]
fn update_role(id:String,input:RoleInput,state:State<'_,AppState>)->Result<Role,String>{
  if input.title.trim().is_empty(){return Err("Chức vụ cần tên.".into())}let c=database_connection(&state.database_path)?;let organization_id=current_organization_id(&c)?;
  if c.execute("UPDATE roles SET title=?2,description=?3 WHERE id=?1 AND organization_id=?4",params![id,input.title.trim(),input.description.trim(),organization_id]).map_err(|e|e.to_string())?==0{return Err("Không tìm thấy chức vụ để cập nhật.".into())};
  Ok(Role{id,organization_id,title:input.title.trim().into(),description:input.description.trim().into()})
}

#[tauri::command]
fn list_runtime_bindings(state: State<'_, AppState>) -> Result<Vec<RuntimeBinding>, String> {
  let connection = database_connection(&state.database_path)?;
  let mut statement = connection.prepare("SELECT id, provider_kind, display_name, status FROM runtime_bindings ORDER BY display_name")
    .map_err(|error| error.to_string())?;
  statement.query_map([], |row| Ok(RuntimeBinding { id: row.get(0)?, provider_kind: row.get(1)?, display_name: row.get(2)?, status: row.get(3)? }))
    .map_err(|error| error.to_string())?.collect::<Result<Vec<_>, _>>().map_err(|error| error.to_string())
}

#[tauri::command]
fn list_employees(state: State<'_, AppState>) -> Result<Vec<Employee>, String> {
  list_employees_from(&database_connection(&state.database_path)?)
}

#[tauri::command]
fn create_employee(input: CreateEmployeeInput, state: State<'_, AppState>) -> Result<Employee, String> {
  if input.display_name.trim().is_empty() { return Err("Tên nhân sự không được để trống.".to_string()); }
  let connection = database_connection(&state.database_path)?;
  ensure_valid_manager(&connection, None, input.manager_employee_id.as_deref())?;
  let id = format!("emp_{}", uuid_suffix()); let organization_id=current_organization_id(&connection)?;
  connection.execute(
    "INSERT INTO employees (id, organization_id, display_name, role_id, manager_employee_id, runtime_binding_id, status, capabilities)
     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
    params![id, organization_id, input.display_name.trim(), input.role_id, input.manager_employee_id, input.runtime_binding_id, input.status, input.capabilities.trim()],
  ).map_err(|error| error.to_string())?;
  list_employees_from(&connection)?.into_iter().find(|employee| employee.id == id)
    .ok_or_else(|| "Không thể đọc lại nhân sự vừa tạo.".to_string())
}

#[tauri::command]
fn update_employee(input: UpdateEmployeeInput, state: State<'_, AppState>) -> Result<Employee, String> {
  if input.display_name.trim().is_empty() { return Err("Tên nhân sự không được để trống.".to_string()); }
  let mut connection = database_connection(&state.database_path)?;
  ensure_valid_manager(&connection, Some(&input.id), input.manager_employee_id.as_deref())?;
  let employee_id=input.id.clone();
  let tx=connection.transaction().map_err(|e|e.to_string())?;
  tx.execute(
    "UPDATE employees SET display_name = ?2, role_id = ?3, manager_employee_id = ?4, runtime_binding_id = ?5, status = ?6, capabilities = ?7 WHERE id = ?1",
    params![employee_id, input.display_name.trim(), input.role_id, input.manager_employee_id, input.runtime_binding_id, input.status, input.capabilities.trim()],
  ).map_err(|error| error.to_string())?;
  tx.execute("INSERT INTO employee_profile_events VALUES(?1,?2,?3,'ceo','ceo_action','employee.profile_updated','CEO cập nhật hồ sơ nhân viên.')",params![format!("eprof_{}",uuid_suffix()),employee_id,now()]).map_err(|e|e.to_string())?;
  tx.commit().map_err(|e|e.to_string())?;
  list_employees_from(&connection)?.into_iter().find(|employee| employee.id == employee_id)
    .ok_or_else(|| "Không tìm thấy nhân sự để cập nhật.".to_string())
}

#[tauri::command]
fn list_employee_profile_events(employee_id:String,state:State<'_,AppState>)->Result<Vec<EmployeeProfileEvent>,String>{
  let c=database_connection(&state.database_path)?;let mut s=c.prepare("SELECT id,employee_id,occurred_at,actor_kind,source_kind,event_type,summary FROM employee_profile_events WHERE employee_id=?1 ORDER BY occurred_at DESC").map_err(|e|e.to_string())?;
  s.query_map([employee_id],profile_event_from_row).map_err(|e|e.to_string())?.collect::<Result<Vec<_>,_>>().map_err(|e|e.to_string())
}

#[tauri::command]
fn create_authority_policy(input:CreateAuthorityPolicyInput,state:State<'_,AppState>)->Result<AuthorityPolicy,String>{
 let mut c=database_connection(&state.database_path)?;create_authority_policy_from(&mut c,input)}
fn create_authority_policy_from(c:&mut Connection,input:CreateAuthorityPolicyInput)->Result<AuthorityPolicy,String>{
 if !has_exactly_one_subject(&input){return Err("Policy phải thuộc chính xác một nhân viên hoặc một chức vụ.".into())} if input.action.trim().is_empty()||input.work_scope.trim().is_empty()||input.resource_scope.trim().is_empty()||input.reason.trim().is_empty(){return Err("Quyền cần action, phạm vi và lý do.".into())};let id=format!("authp_{}",uuid_suffix());let event_reason=input.reason.clone();let tx=c.transaction().map_err(|e|e.to_string())?;tx.execute("INSERT INTO authority_policy_versions VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,'active',NULL,?11)",params![id,input.employee_id,input.role_id,input.work_scope,input.action,input.resource_scope,input.expires_at,input.quota,input.can_delegate,input.can_self_approve,input.reason]).map_err(|e|e.to_string())?;tx.execute("INSERT INTO authority_policy_events VALUES(?1,?2,?3,'ceo','ceo_action','authority.granted',?4,NULL)",params![format!("apol_evt_{}",uuid_suffix()),id,now(),event_reason]).map_err(|e|e.to_string())?;tx.commit().map_err(|e|e.to_string())?;c.query_row("SELECT id,employee_id,role_id,work_scope,action,resource_scope,expires_at,quota,can_delegate,can_self_approve,status,supersedes_id,reason FROM authority_policy_versions WHERE id=?1",[id],policy_from_row).map_err(|e|e.to_string())}
#[tauri::command]
fn list_authority_policies(employee_id:String,state:State<'_,AppState>)->Result<Vec<AuthorityPolicy>,String>{list_authority_policies_from(&database_connection(&state.database_path)?,&employee_id)}
fn list_authority_policies_from(c:&Connection,employee_id:&str)->Result<Vec<AuthorityPolicy>,String>{let mut s=c.prepare("SELECT p.id,p.employee_id,p.role_id,p.work_scope,p.action,p.resource_scope,p.expires_at,p.quota,p.can_delegate,p.can_self_approve,p.status,p.supersedes_id,p.reason FROM authority_policy_versions p LEFT JOIN employees e ON p.employee_id IS NULL AND e.role_id=p.role_id WHERE p.employee_id=?1 OR (p.employee_id IS NULL AND e.id=?1) ORDER BY p.id DESC").map_err(|e|e.to_string())?;s.query_map([employee_id],policy_from_row).map_err(|e|e.to_string())?.collect::<Result<Vec<_>,_>>().map_err(|e|e.to_string())}
#[tauri::command]
fn narrow_authority_policy(id:String,input:CreateAuthorityPolicyInput,state:State<'_,AppState>)->Result<AuthorityPolicy,String>{let mut c=database_connection(&state.database_path)?;narrow_authority_policy_from(&mut c,&id,input)}
fn narrow_authority_policy_from(c:&mut Connection,id:&str,input:CreateAuthorityPolicyInput)->Result<AuthorityPolicy,String>{let old=c.query_row("SELECT id,employee_id,role_id,work_scope,action,resource_scope,expires_at,quota,can_delegate,can_self_approve,status,supersedes_id,reason FROM authority_policy_versions WHERE id=?1 AND status='active'",[id],policy_from_row).map_err(|_|"Không tìm thấy quyền active để thu hẹp.".to_string())?;if !has_exactly_one_subject(&input)||!is_narrower(&old,&input){return Err("Policy kế nhiệm phải giữ đúng chủ thể và là tập con của policy cũ.".into())};let tx=c.transaction().map_err(|e|e.to_string())?;tx.execute("UPDATE authority_policy_versions SET status='narrowed' WHERE id=?1",[id]).map_err(|e|e.to_string())?;let nid=format!("authp_{}",uuid_suffix());tx.execute("INSERT INTO authority_policy_versions VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,'active',?11,?12)",params![nid,input.employee_id,input.role_id,input.work_scope,input.action,input.resource_scope,input.expires_at,input.quota,input.can_delegate,input.can_self_approve,id,input.reason]).map_err(|e|e.to_string())?;tx.execute("INSERT INTO authority_policy_events VALUES(?1,?2,?3,'ceo','ceo_action','authority.narrowed',?4,?5)",params![format!("apol_evt_{}",uuid_suffix()),id,now(),"Policy được thu hẹp.",&nid]).map_err(|e|e.to_string())?;tx.execute("INSERT INTO authority_policy_events VALUES(?1,?2,?3,'ceo','ceo_action','authority.granted',?4,?5)",params![format!("apol_evt_{}",uuid_suffix()),nid,now(),input.reason,id]).map_err(|e|e.to_string())?;tx.commit().map_err(|e|e.to_string())?;c.query_row("SELECT id,employee_id,role_id,work_scope,action,resource_scope,expires_at,quota,can_delegate,can_self_approve,status,supersedes_id,reason FROM authority_policy_versions WHERE id=?1",[nid],policy_from_row).map_err(|e|e.to_string())}
#[tauri::command]
fn revoke_authority_policy(input:RevokeAuthorityPolicyInput,state:State<'_,AppState>)->Result<(),String>{let mut c=database_connection(&state.database_path)?;revoke_authority_policy_from(&mut c,input)}
fn revoke_authority_policy_from(c:&mut Connection,input:RevokeAuthorityPolicyInput)->Result<(),String>{
 if input.reason.trim().is_empty(){return Err("Thu hồi quyền cần lý do.".into())}
 let tx=c.transaction().map_err(|e|e.to_string())?;
 let status:Option<String>=tx.query_row("SELECT status FROM authority_policy_versions WHERE id=?1",[&input.id],|r|r.get(0)).optional().map_err(|e|e.to_string())?;
 match status.as_deref(){
   None=>return Err(format!("Không tìm thấy policy `{}` để thu hồi.",input.id)),
   Some("active")=>{},
   Some(current)=>return Err(format!("Policy `{}` đang có status `{}`; chỉ policy `active` mới có thể thu hồi. Hãy chọn policy kế nhiệm đang active (nếu có).",input.id,current)),
 }
 if tx.execute("UPDATE authority_policy_versions SET status='revoked' WHERE id=?1 AND status='active'",[&input.id]).map_err(|e|e.to_string())?!=1{return Err(format!("Không thể chuyển policy `{}` sang revoked.",input.id))}
 tx.execute("INSERT INTO authority_policy_events VALUES(?1,?2,?3,'ceo','ceo_action','authority.revoked',?4,NULL)",params![format!("apol_evt_{}",uuid_suffix()),input.id,now(),input.reason]).map_err(|e|e.to_string())?;
 tx.commit().map_err(|e|e.to_string())?;Ok(())
}
#[tauri::command]
fn list_authority_policy_events(employee_id:String,state:State<'_,AppState>)->Result<Vec<AuthorityPolicyEvent>,String>{let c=database_connection(&state.database_path)?;let mut s=c.prepare("SELECT DISTINCT ev.id,ev.policy_id,ev.occurred_at,ev.actor_kind,ev.source_kind,ev.event_type,ev.reason,ev.related_policy_id FROM authority_policy_events ev JOIN authority_policy_versions p ON p.id=ev.policy_id LEFT JOIN employees e ON p.employee_id IS NULL AND e.role_id=p.role_id WHERE p.employee_id=?1 OR (p.employee_id IS NULL AND e.id=?1) ORDER BY ev.occurred_at DESC").map_err(|e|e.to_string())?;s.query_map([employee_id],policy_event_from_row).map_err(|e|e.to_string())?.collect::<Result<Vec<_>,_>>().map_err(|e|e.to_string())}
#[tauri::command]
fn evaluate_authority_policy(input:EvaluatePolicyInput,state:State<'_,AppState>)->Result<AuthorityDecision,String>{evaluate_policy_from(&database_connection(&state.database_path)?,input)}
fn evaluate_policy_from(c:&Connection,input:EvaluatePolicyInput)->Result<AuthorityDecision,String>{let mut s=c.prepare("SELECT p.id,p.employee_id,p.role_id,p.work_scope,p.action,p.resource_scope,p.expires_at,p.quota,p.can_delegate,p.can_self_approve,p.status,p.supersedes_id,p.reason FROM authority_policy_versions p LEFT JOIN employees e ON p.employee_id IS NULL AND e.role_id=p.role_id WHERE (p.employee_id=?1 OR (p.employee_id IS NULL AND e.id=?1)) AND p.status='active' ORDER BY CASE WHEN p.employee_id IS NULL THEN 1 ELSE 0 END, p.id DESC").map_err(|e|e.to_string())?;let all=s.query_map([&input.employee_id],policy_from_row).map_err(|e|e.to_string())?.collect::<Result<Vec<_>,_>>().map_err(|e|e.to_string())?;let employee_override=all.iter().any(|p|p.employee_id.as_deref()==Some(&input.employee_id)&&p.action==input.action);for p in all{if employee_override&&p.employee_id.as_deref()!=Some(&input.employee_id){continue}let time=p.expires_at.as_ref().map(|x|x>&now()).unwrap_or(true);let quota=p.quota.map(|x|input.requested_quota.unwrap_or(0.)<=x).unwrap_or(true);if scope_includes(&p.work_scope,&input.work_scope)&&p.action==input.action&&scope_includes(&p.resource_scope,&input.resource_scope)&&time&&quota&&(!input.requires_delegate||p.can_delegate)&&(!input.requires_self_approve||p.can_self_approve){let source=if p.employee_id.is_some(){"employee"}else{"role"};return Ok(AuthorityDecision{allowed:true,reason_code:"allowed".into(),reason:format!("Được phép bởi {}",p.id),policy_id:Some(p.id),policy_source:Some(source.into()),matched_limits:Some(format!("work={}, resource={}, quota={:?}, delegate={}, self_approve={}",p.work_scope,p.resource_scope,p.quota,p.can_delegate,p.can_self_approve))})}};Ok(AuthorityDecision{allowed:false,reason_code:if employee_override{"employee_override_denied"}else{"no_matching_policy"}.into(),reason:"Không có policy active khớp request; role policy không thể vượt employee override giới hạn.".into(),policy_id:None,policy_source:None,matched_limits:None})}

#[tauri::command]
fn append_ledger_event(input: AppendLedgerInput, state: State<'_, AppState>) -> Result<LedgerEvent, String> { append_event(&database_connection(&state.database_path)?, input) }
#[tauri::command]
fn list_ledger_events(state: State<'_, AppState>) -> Result<Vec<LedgerEvent>, String> {
  let connection = database_connection(&state.database_path)?; let mut statement = connection.prepare("SELECT id, occurred_at, recorded_at, event_type, actor_kind, actor_employee_id, source_kind, summary, details_json, correction_of_event_id FROM ledger_events ORDER BY recorded_at DESC").map_err(|e| e.to_string())?;
  statement.query_map([], ledger_from_row).map_err(|e| e.to_string())?.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())
}
#[tauri::command]
fn create_report(input: CreateReportInput, state: State<'_, AppState>) -> Result<Report, String> {
  if input.title.trim().is_empty() || input.work_done.trim().is_empty() || input.result.trim().is_empty() { return Err("Report cần tiêu đề, việc đã làm và kết quả.".to_string()); }
  let connection = database_connection(&state.database_path)?; let id = format!("rpt_{}", uuid_suffix()); let created_at = now();
  connection.execute("INSERT INTO reports VALUES (?1, ?2, 'submitted', ?3, ?4, ?5, ?6, ?7, ?8)", params![id, input.title, input.work_done, input.result, input.uncertainties, input.delegation_summary, input.next_steps, created_at]).map_err(|e| e.to_string())?;
  if let Some(location) = input.evidence_location.filter(|value| !value.trim().is_empty()) { let artifact_id=format!("art_{}", uuid_suffix()); connection.execute("INSERT INTO artifacts VALUES (?1, ?2, ?3, ?4, ?5)", params![artifact_id, id, location, "unverified", now()]).map_err(|e| e.to_string())?; }
  append_event(&connection, AppendLedgerInput { event_type: "report.submitted".into(), actor_kind: "ceo".into(), actor_employee_id: None, source_kind: "user_action".into(), summary: format!("Đã nộp report {}", id), details_json: "{}".into(), correction_of_event_id: None })?;
  connection.query_row("SELECT id,title,status,work_done,result,uncertainties,delegation_summary,next_steps,created_at FROM reports WHERE id=?1", [id], |r| Ok(Report { id:r.get(0)?, title:r.get(1)?, status:r.get(2)?, work_done:r.get(3)?, result:r.get(4)?, uncertainties:r.get(5)?, delegation_summary:r.get(6)?, next_steps:r.get(7)?, created_at:r.get(8)? })).map_err(|e| e.to_string())
}
#[tauri::command]
fn list_reports(state: State<'_, AppState>) -> Result<Vec<Report>, String> { let connection=database_connection(&state.database_path)?; let mut s=connection.prepare("SELECT id,title,status,work_done,result,uncertainties,delegation_summary,next_steps,created_at FROM reports ORDER BY created_at DESC").map_err(|e|e.to_string())?; s.query_map([], |r| Ok(Report{id:r.get(0)?,title:r.get(1)?,status:r.get(2)?,work_done:r.get(3)?,result:r.get(4)?,uncertainties:r.get(5)?,delegation_summary:r.get(6)?,next_steps:r.get(7)?,created_at:r.get(8)?})).map_err(|e|e.to_string())?.collect::<Result<Vec<_>,_>>().map_err(|e|e.to_string()) }

fn uuid_suffix() -> String {
  use std::time::{SystemTime, UNIX_EPOCH};
  format!("{}_{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos(), ID_COUNTER.fetch_add(1, Ordering::Relaxed))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      let directory = app.path().app_local_data_dir()?;
      fs::create_dir_all(&directory)?;
      let database_path = directory.join("aicorp.sqlite3");
      database_connection(&database_path).map_err(std::io::Error::other)?;
      app.manage(AppState { database_path });
      if cfg!(debug_assertions) {
        app.handle().plugin(tauri_plugin_log::Builder::default().level(log::LevelFilter::Info).build())?;
      }
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![initialize_demo_organization, create_organization, update_organization, get_organization, list_roles, create_role, update_role, list_runtime_bindings, list_employees, create_employee, update_employee, list_employee_profile_events, create_authority_policy, list_authority_policies, narrow_authority_policy, revoke_authority_policy, list_authority_policy_events, evaluate_authority_policy, append_ledger_event, list_ledger_events, create_report, list_reports])
    .run(tauri::generate_context!())
    .expect("error while building tauri application");
}

#[cfg(test)]
mod tests {
  use super::*;
  fn test_database() -> (PathBuf, Connection) {
    let path = std::env::temp_dir().join(format!("aicorp-test-{}.sqlite3", uuid_suffix()));
    let connection = database_connection(&path).expect("create local database");
    ensure_sample_data(&connection).expect("seed organization");
    connection.execute("INSERT INTO roles VALUES ('role_research','org_aicorp_hq','Research Agent','Research')", []).expect("second role");
    connection.execute("INSERT INTO employees (id,organization_id,display_name,role_id,manager_employee_id,runtime_binding_id,status,capabilities) VALUES ('emp_research','org_aicorp_hq','Binh','role_research',NULL,NULL,'active','')", []).expect("second employee");
    (path, connection)
  }
  fn add_policy(c:&Connection,id:&str,employee:Option<&str>,role:Option<&str>,action:&str,work:&str,resource:&str,quota:Option<f64>,delegate:bool,self_approve:bool,status:&str,expires:Option<&str>){
    c.execute("INSERT INTO authority_policy_versions VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,NULL,'test policy')",params![id,employee,role,work,action,resource,expires,quota,delegate,self_approve,status]).expect("insert policy");
  }
  fn request(employee:&str,action:&str,work:&str,resource:&str,quota:Option<f64>,delegate:bool,self_approve:bool)->EvaluatePolicyInput{EvaluatePolicyInput{employee_id:employee.into(),action:action.into(),work_scope:work.into(),resource_scope:resource.into(),requested_quota:quota,requires_delegate:delegate,requires_self_approve:self_approve}}
  fn cleanup(path:PathBuf,connection:Connection){drop(connection);std::fs::remove_file(path).expect("remove test database");}

  #[test]
  fn policy_subject_is_exactly_one_and_role_scope_is_isolated() {
    let (path,c)=test_database();
    assert!(c.execute("INSERT INTO authority_policy_versions VALUES ('bad_both','emp_ops_001','role_operations','*','execute','*',NULL,NULL,0,0,'active',NULL,'bad')", []).is_err());
    assert!(c.execute("INSERT INTO authority_policy_versions VALUES ('bad_none',NULL,NULL,'*','execute','*',NULL,NULL,0,0,'active',NULL,'bad')", []).is_err());
    add_policy(&c,"role_ops",None,Some("role_operations"),"execute","*","*",None,true,true,"active",None);
    let allowed=evaluate_policy_from(&c,request("emp_ops_001","execute","report","local",None,false,false)).expect("role decision");
    assert!(allowed.allowed); assert_eq!(allowed.policy_source.as_deref(),Some("role")); assert_eq!(allowed.policy_id.as_deref(),Some("role_ops"));
    let denied=evaluate_policy_from(&c,request("emp_research","execute","report","local",None,false,false)).expect("wrong role decision");
    assert!(!denied.allowed); assert_eq!(denied.reason_code,"no_matching_policy");
    cleanup(path,c);
  }

  #[test]
  fn evaluator_returns_metadata_and_enforces_limits_and_override() {
    let (path,c)=test_database();
    add_policy(&c,"role_broad",None,Some("role_operations"),"execute","*","*",None,true,true,"active",None);
    add_policy(&c,"employee_limited",Some("emp_ops_001"),None,"execute","work/*","local/*",Some(10.0),true,true,"active",None);
    let allow=evaluate_policy_from(&c,request("emp_ops_001","execute","work/sub","local/sub",Some(5.0),true,true)).expect("allow");
    assert!(allow.allowed); assert_eq!(allow.reason_code,"allowed"); assert_eq!(allow.policy_id.as_deref(),Some("employee_limited")); assert_eq!(allow.policy_source.as_deref(),Some("employee")); assert!(allow.matched_limits.unwrap().contains("self_approve=true"));
    let over_quota=evaluate_policy_from(&c,request("emp_ops_001","execute","work/sub","local/sub",Some(11.0),false,false)).expect("quota");
    assert!(!over_quota.allowed); assert_eq!(over_quota.reason_code,"employee_override_denied");
    let wrong_scope=evaluate_policy_from(&c,request("emp_ops_001","execute","outside","local/sub",None,false,false)).expect("scope");
    assert!(!wrong_scope.allowed); assert_eq!(wrong_scope.reason_code,"employee_override_denied");
    cleanup(path,c);
  }

  #[test]
  fn expiry_revoke_and_legacy_migration_do_not_bypass_policy_evaluator() {
    let (path,c)=test_database();
    add_policy(&c,"expired",Some("emp_ops_001"),None,"prepare","*","*",None,false,false,"active",Some("2000-01-01T00:00:00Z"));
    let expired=evaluate_policy_from(&c,request("emp_ops_001","prepare","x","x",None,false,false)).expect("expired");
    assert!(!expired.allowed); assert_eq!(expired.reason_code,"employee_override_denied");
    add_policy(&c,"revoked",Some("emp_ops_001"),None,"review","*","*",None,false,false,"revoked",None);
    let revoked=evaluate_policy_from(&c,request("emp_ops_001","review","x","x",None,false,false)).expect("revoked");
    assert!(!revoked.allowed); assert_eq!(revoked.reason_code,"no_matching_policy");
    c.execute("INSERT INTO authority_envelopes VALUES ('legacy_one','emp_ops_001','prepare_report','reports/*','weekly',NULL,5.0,'delegate',1,'active')", []).expect("legacy row");
    drop(c); let reopened=database_connection(&path).expect("reopen migrates legacy");
    let migrated:i64=reopened.query_row("SELECT COUNT(*) FROM authority_policy_versions WHERE id='authp_migrated_legacy_one'",[],|r|r.get(0)).expect("migrated policy"); assert_eq!(migrated,1);
    cleanup(path,reopened);
  }

  #[test]
  fn narrow_preserves_subject_records_history_and_rolls_back_on_failure() {
    let (path,mut c)=test_database();
    add_policy(&c,"narrow_me",Some("emp_ops_001"),None,"execute","work/*","local/*",Some(10.0),true,true,"active",None);
    let wrong_subject=CreateAuthorityPolicyInput{employee_id:None,role_id:Some("role_operations".into()),work_scope:"work/sub".into(),action:"execute".into(),resource_scope:"local/sub".into(),expires_at:None,quota:Some(5.0),can_delegate:false,can_self_approve:false,reason:"wrong subject".into()};
    assert!(narrow_authority_policy_from(&mut c,"narrow_me",wrong_subject).is_err());
    let unchanged:String=c.query_row("SELECT status FROM authority_policy_versions WHERE id='narrow_me'",[],|r|r.get(0)).expect("old active"); assert_eq!(unchanged,"active");
    let valid=CreateAuthorityPolicyInput{employee_id:Some("emp_ops_001".into()),role_id:None,work_scope:"work/sub".into(),action:"execute".into(),resource_scope:"local/sub".into(),expires_at:None,quota:Some(5.0),can_delegate:false,can_self_approve:false,reason:"narrow valid".into()};
    let successor=narrow_authority_policy_from(&mut c,"narrow_me",valid).expect("narrow"); assert_eq!(successor.supersedes_id.as_deref(),Some("narrow_me"));
    let events:i64=c.query_row("SELECT COUNT(*) FROM authority_policy_events WHERE policy_id IN ('narrow_me',?1)",[&successor.id],|r|r.get(0)).expect("history"); assert_eq!(events,2);
    add_policy(&c,"rollback_me",Some("emp_ops_001"),None,"delete","*","*",None,false,false,"active",None);
    c.execute_batch("CREATE TRIGGER block_successor BEFORE INSERT ON authority_policy_versions WHEN NEW.supersedes_id IS NOT NULL BEGIN SELECT RAISE(ABORT,'forced successor failure'); END;").expect("failure trigger");
    let rollback_input=CreateAuthorityPolicyInput{employee_id:Some("emp_ops_001".into()),role_id:None,work_scope:"delete/sub".into(),action:"delete".into(),resource_scope:"local/sub".into(),expires_at:None,quota:None,can_delegate:false,can_self_approve:false,reason:"force rollback".into()};
    assert!(narrow_authority_policy_from(&mut c,"rollback_me",rollback_input).is_err());
    let rollback_status:String=c.query_row("SELECT status FROM authority_policy_versions WHERE id='rollback_me'",[],|r|r.get(0)).expect("rollback status"); assert_eq!(rollback_status,"active");
    cleanup(path,c);
  }

  #[test]
  fn revoke_successor_records_history_and_rolls_back_when_revoke_event_fails() {
    let (path,mut c)=test_database();
    add_policy(&c,"revoke_parent",Some("emp_ops_001"),None,"execute","work/*","local/*",Some(10.0),true,true,"active",None);
    let narrow_input=CreateAuthorityPolicyInput{employee_id:Some("emp_ops_001".into()),role_id:None,work_scope:"work/sub".into(),action:"execute".into(),resource_scope:"local/sub".into(),expires_at:None,quota:Some(5.0),can_delegate:false,can_self_approve:false,reason:"narrow before revoke".into()};
    let successor=narrow_authority_policy_from(&mut c,"revoke_parent",narrow_input).expect("narrow for revoke");
    let narrowed_error=revoke_authority_policy_from(&mut c,RevokeAuthorityPolicyInput{id:"revoke_parent".into(),reason:"wrong target".into()}).expect_err("narrowed parent cannot be revoked");
    assert!(narrowed_error.contains("revoke_parent")); assert!(narrowed_error.contains("narrowed"));
    revoke_authority_policy_from(&mut c,RevokeAuthorityPolicyInput{id:successor.id.clone(),reason:"CEO stops delegated work".into()}).expect("revoke active successor");
    let parent_status:String=c.query_row("SELECT status FROM authority_policy_versions WHERE id='revoke_parent'",[],|r|r.get(0)).expect("parent status");
    let successor_status:String=c.query_row("SELECT status FROM authority_policy_versions WHERE id=?1",[&successor.id],|r|r.get(0)).expect("successor status");
    let revoke_event:(String,String)=c.query_row("SELECT event_type,reason FROM authority_policy_events WHERE policy_id=?1 AND event_type='authority.revoked'",[&successor.id],|r|Ok((r.get(0)?,r.get(1)?))).expect("revoke history");
    assert_eq!(parent_status,"narrowed"); assert_eq!(successor_status,"revoked"); assert_eq!(revoke_event.0,"authority.revoked"); assert_eq!(revoke_event.1,"CEO stops delegated work");
    add_policy(&c,"revoke_rollback",Some("emp_ops_001"),None,"review","*","*",None,false,false,"active",None);
    c.execute_batch("CREATE TRIGGER block_revoke_event BEFORE INSERT ON authority_policy_events WHEN NEW.event_type='authority.revoked' BEGIN SELECT RAISE(ABORT,'forced revoke event failure'); END;").expect("revoke event trigger");
    assert!(revoke_authority_policy_from(&mut c,RevokeAuthorityPolicyInput{id:"revoke_rollback".into(),reason:"must rollback".into()}).is_err());
    let rollback_status:String=c.query_row("SELECT status FROM authority_policy_versions WHERE id='revoke_rollback'",[],|r|r.get(0)).expect("rollback policy status");
    let rollback_events:i64=c.query_row("SELECT COUNT(*) FROM authority_policy_events WHERE policy_id='revoke_rollback'",[],|r|r.get(0)).expect("rollback event count");
    assert_eq!(rollback_status,"active"); assert_eq!(rollback_events,0);
    cleanup(path,c);
  }

  #[test]
  fn runtime_labels_reject_secret_like_values_and_ledger_remains_append_only() {
    assert!(validate_runtime_display_field("Local runtime","display name").is_ok());
    assert!(validate_runtime_display_field("sk-live-secret","display name").is_err());
    assert!(validate_runtime_display_field("Bearer token","provider").is_err());
    let (path,c)=test_database();
    let original=append_event(&c,AppendLedgerInput{event_type:"employee.created".into(),actor_kind:"system".into(),actor_employee_id:None,source_kind:"system".into(),summary:"seed".into(),details_json:"{}".into(),correction_of_event_id:None}).expect("append ledger");
    assert!(c.execute("UPDATE ledger_events SET summary='changed' WHERE id=?1",[&original.id]).is_err());
    assert!(c.execute("DELETE FROM ledger_events WHERE id=?1",[&original.id]).is_err());
    cleanup(path,c);
  }

  #[test]
  fn grant_and_legacy_migration_rollback_when_policy_event_fails() {
    let (path,mut c)=test_database();
    c.execute_batch("CREATE TRIGGER block_policy_event BEFORE INSERT ON authority_policy_events BEGIN SELECT RAISE(ABORT,'forced policy event failure'); END;").expect("event failure trigger");
    let grant=CreateAuthorityPolicyInput{employee_id:Some("emp_ops_001".into()),role_id:None,work_scope:"*".into(),action:"execute".into(),resource_scope:"*".into(),expires_at:None,quota:None,can_delegate:false,can_self_approve:false,reason:"must rollback".into()};
    assert!(create_authority_policy_from(&mut c,grant).is_err());
    let grant_count:i64=c.query_row("SELECT COUNT(*) FROM authority_policy_versions WHERE action='execute'",[],|r|r.get(0)).expect("grant rollback"); assert_eq!(grant_count,0);
    c.execute("INSERT INTO authority_envelopes VALUES ('legacy_rollback','emp_ops_001','migrate','*','*',NULL,NULL,'none',0,'active')",[]).expect("legacy row");
    assert!(migrate_legacy_authority_envelopes(&mut c).is_err());
    let migration_count:i64=c.query_row("SELECT COUNT(*) FROM authority_policy_versions WHERE id='authp_migrated_legacy_rollback'",[],|r|r.get(0)).expect("migration rollback"); assert_eq!(migration_count,0);
    cleanup(path,c);
  }

  #[test]
  fn runtime_seed_boundary_and_profile_history_have_explicit_states() {
    let (path,c)=test_database();
    assert!(insert_seed_runtime_binding(&c,"bad_runtime","local","sk-secret","local://private","available").is_err());
    let unsafe_count:i64=c.query_row("SELECT COUNT(*) FROM runtime_bindings WHERE id='bad_runtime'",[],|r|r.get(0)).expect("runtime rejected"); assert_eq!(unsafe_count,0);
    let empty_history:i64=c.query_row("SELECT COUNT(*) FROM employee_profile_events WHERE employee_id='emp_ops_001'",[],|r|r.get(0)).expect("empty initial history"); assert_eq!(empty_history,0);
    append_profile_event(&c,"emp_ops_001","employee.profile_updated","CEO updated profile").expect("profile event");
    assert!(c.execute("UPDATE employee_profile_events SET summary='changed'",[]).is_err());
    cleanup(path,c);
  }

  #[test]
  fn organization_role_capabilities_and_profile_history_persist_after_reopen() {
    let (path,c)=test_database();
    c.execute("UPDATE organizations SET name='Verified Corp', ceo_display_name='CEO Test' WHERE id='org_aicorp_hq'",[]).expect("update organization");
    c.execute("INSERT INTO roles VALUES ('role_quality','org_aicorp_hq','Quality Agent','Checks deliverables')",[]).expect("insert role");
    c.execute("INSERT INTO employees (id,organization_id,display_name,role_id,manager_employee_id,runtime_binding_id,status,capabilities) VALUES ('emp_quality','org_aicorp_hq','Chi','role_quality',NULL,NULL,'active','review reports, test flows')",[]).expect("insert employee");
    append_profile_event(&c,"emp_quality","employee.profile_updated","CEO declared capabilities").expect("profile history");
    drop(c); let reopened=database_connection(&path).expect("reopen database");
    let organization_name:String=reopened.query_row("SELECT name FROM organizations WHERE id='org_aicorp_hq'",[],|r|r.get(0)).expect("organization persisted"); assert_eq!(organization_name,"Verified Corp");
    let employee=list_employees_from(&reopened).expect("employees").into_iter().find(|item|item.id=="emp_quality").expect("quality employee"); assert_eq!(employee.role_title,"Quality Agent"); assert_eq!(employee.capabilities,"review reports, test flows");
    let history:i64=reopened.query_row("SELECT COUNT(*) FROM employee_profile_events WHERE employee_id='emp_quality'",[],|r|r.get(0)).expect("profile history persisted"); assert_eq!(history,1);
    cleanup(path,reopened);
  }
}
