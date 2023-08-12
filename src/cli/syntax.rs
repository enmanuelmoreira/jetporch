// Jetporch
// Copyright (C) 2023 - Michael DeHaan <michael@michaeldehaan.net> + contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// at your option) any later version.
// 
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// long with this program.  If not, see <http://www.gnu.org/licenses/>.

use crate::playbooks::traversal::{playbook_traversal};
use crate::connection::no::{NoFactory};
use crate::playbooks::context::{PlaybookContext};
use std::path::PathBuf;
use crate::playbooks::visitor::PlaybookVisitor;

struct SyntaxVisitor {
}

impl PlaybookVisitor for SyntaxVisitor {
}

impl SyntaxVisitor {
    pub fn new() -> Self {
        Self {}
    }
}

pub fn playbook_syntax_scan(playbook_paths: &Vec<PathBuf>) -> Result<(), String> {
    
    let mut context = PlaybookContext::new();
    let visitor = SyntaxVisitor::new();
    let factory = NoFactory::new();

    return playbook_traversal(&playbook_paths, &mut context, &visitor, &factory);

}