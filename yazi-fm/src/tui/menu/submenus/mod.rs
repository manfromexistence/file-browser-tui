// Submenu definitions for all command palette items
mod theme;
pub(super) mod keyboard_shortcuts;
mod providers;
mod plugins_apps;
mod skills;
mod sandbox;
mod web_search;
mod mcp_servers;
mod memory_history;
mod multi_agent;
mod notifications;
mod voice_realtime;
mod image_vision;
mod profiles;
mod worktree;
mod authentication;
mod network_proxy;
mod hooks_events;
mod session_resume;
mod approval_policy;
mod shell_environment;
mod execution_rules;
mod project_trust;
mod developer_instructions;
mod feature_flags;

pub fn get_all_submenus() -> Vec<Vec<(&'static str, &'static str)>> {
    vec![
        theme::get_submenu(),
        keyboard_shortcuts::get_submenu(),
        providers::get_submenu(),
        plugins_apps::get_submenu(),
        skills::get_submenu(),
        sandbox::get_submenu(),
        web_search::get_submenu(),
        mcp_servers::get_submenu(),
        memory_history::get_submenu(),
        multi_agent::get_submenu(),
        notifications::get_submenu(),
        voice_realtime::get_submenu(),
        image_vision::get_submenu(),
        profiles::get_submenu(),
        worktree::get_submenu(),
        authentication::get_submenu(),
        network_proxy::get_submenu(),
        hooks_events::get_submenu(),
        session_resume::get_submenu(),
        approval_policy::get_submenu(),
        shell_environment::get_submenu(),
        execution_rules::get_submenu(),
        project_trust::get_submenu(),
        developer_instructions::get_submenu(),
        feature_flags::get_submenu(),
    ]
}
