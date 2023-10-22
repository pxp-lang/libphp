#include "wrapper.h"

uint8_t libphp_zval_get_type(const zval* pz) {
    return zval_get_type(pz);
}

const char *libphp_zval_get_string(zval *pz)
{
    convert_to_string(pz);
    return Z_STRVAL_P(pz);
}

const char* libphp_var_export(zval *pz) 
{
    smart_str buf = {0};
    php_var_export_ex(pz, 1, &buf);
    smart_str_0(&buf);

    const char* exported = buf.s->val;
    smart_str_free(&buf); 

    return exported;
}

void libphp_zval_create_string(zval *pz, const char *str)
{
    ZVAL_STRING_FAST(pz, str);
}

void libphp_register_variable(const char *key, zval *value)
{
    zend_hash_str_update(&EG(symbol_table), key, strlen(key), value);
}