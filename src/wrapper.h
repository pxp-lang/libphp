#include "Zend/zend.h"
#include "Zend/zend_API.h"
#include "main/php.h"
#include "sapi/embed/php_embed.h"
#include "Zend/zend_compile.h"
#include <Zend/zend_types.h>
#include <ext/standard/php_var.h>
#include "zend_smart_str.h"

uint8_t libphp_zval_get_type(const zval*);

const char* libphp_zval_get_string(zval*);

const char* libphp_var_export(zval *pz);