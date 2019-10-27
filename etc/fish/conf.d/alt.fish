set -l shims_dir "$HOME/.local/alt/shims"

if test "$fish_user_paths[1]" != "$shims_dir"
  # Remove all other instances of the shims dir in the PATH
  while set -l shims_dir_pos (contains -i -- "$shims_dir" $fish_user_paths)
    set -e "fish_user_paths[$shims_dir_pos]"
  end

  set -U fish_user_paths "$shims_dir" $fish_user_paths
end
