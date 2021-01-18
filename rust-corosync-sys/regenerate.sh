
regen()
{
    bindgen --size_t-is-usize --no-recursive-whitelist --no-prepend-enum-name --no-layout-tests --no-doc-comments --generate functions,types /usr/include/corosync/$1.h -o src/$1.rs
}


regen cpg
regen cfg
regen cmap
regen quorum
regen votequorum

