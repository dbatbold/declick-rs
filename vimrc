noremap <leader>cc :!cargo check --color=always 2>&1 \| less -R<cr>
noremap <leader>rf :!rustfmt %  2>&1 \| less -R<cr>
