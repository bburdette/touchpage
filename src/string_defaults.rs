

pub const SAMPLE_GUI_CONFIG: &'static str = r##"{
  "title": "rustmeh",
  "rootControl": 
    {
      "type": "sizer",
      "orientation": "vertical",
      "controls": [
        {
           "type": "sizer",
           "orientation": "horizontal",
           "controls": [
             { "type": "slider"
             , "orientation": "horizontal"
             , "name": "hs1"
             }
            ,{ "type": "slider"
             , "orientation": "horizontal"
             , "name": "hs2"
             }
            ]
         }
       , {
           "type": "sizer",
           "orientation": "horizontal",
           "controls": [
              { "type": "label"
              , "name": "lb1"
              , "label": "blah"
              }
            , { "type": "label"
              , "name": "lb2"
              , "label": "blah"
              }
            ]
        }
      , { "type": "label"
        , "name": "lb3"
        , "label": "blah"
        }
      , { "type": "button"
        , "name": "b1"
        }
      , { "type": "button"
        , "name": "b2"
        }
      , { "type": "button"
        , "name": "b3"
        }
       ]
    }
}"##;


pub const MAIN_HTML: &'static str = r##"<!DOCTYPE HTML>
<html>
<head>
  <meta charset="UTF-8">
  <title>Main</title>
  <style> body { margin: 0; }</style>
</head>
<body>
<div id="elm"></div>
<script>(function(scope){
'use strict';

function F(arity, fun, wrapper) {
  wrapper.a = arity;
  wrapper.f = fun;
  return wrapper;
}

function F2(fun) {
  return F(2, fun, function(a) { return function(b) { return fun(a,b); }; })
}
function F3(fun) {
  return F(3, fun, function(a) {
    return function(b) { return function(c) { return fun(a, b, c); }; };
  });
}
function F4(fun) {
  return F(4, fun, function(a) { return function(b) { return function(c) {
    return function(d) { return fun(a, b, c, d); }; }; };
  });
}
function F5(fun) {
  return F(5, fun, function(a) { return function(b) { return function(c) {
    return function(d) { return function(e) { return fun(a, b, c, d, e); }; }; }; };
  });
}
function F6(fun) {
  return F(6, fun, function(a) { return function(b) { return function(c) {
    return function(d) { return function(e) { return function(f) {
    return fun(a, b, c, d, e, f); }; }; }; }; };
  });
}
function F7(fun) {
  return F(7, fun, function(a) { return function(b) { return function(c) {
    return function(d) { return function(e) { return function(f) {
    return function(g) { return fun(a, b, c, d, e, f, g); }; }; }; }; }; };
  });
}
function F8(fun) {
  return F(8, fun, function(a) { return function(b) { return function(c) {
    return function(d) { return function(e) { return function(f) {
    return function(g) { return function(h) {
    return fun(a, b, c, d, e, f, g, h); }; }; }; }; }; }; };
  });
}
function F9(fun) {
  return F(9, fun, function(a) { return function(b) { return function(c) {
    return function(d) { return function(e) { return function(f) {
    return function(g) { return function(h) { return function(i) {
    return fun(a, b, c, d, e, f, g, h, i); }; }; }; }; }; }; }; };
  });
}

function A2(fun, a, b) {
  return fun.a === 2 ? fun.f(a, b) : fun(a)(b);
}
function A3(fun, a, b, c) {
  return fun.a === 3 ? fun.f(a, b, c) : fun(a)(b)(c);
}
function A4(fun, a, b, c, d) {
  return fun.a === 4 ? fun.f(a, b, c, d) : fun(a)(b)(c)(d);
}
function A5(fun, a, b, c, d, e) {
  return fun.a === 5 ? fun.f(a, b, c, d, e) : fun(a)(b)(c)(d)(e);
}
function A6(fun, a, b, c, d, e, f) {
  return fun.a === 6 ? fun.f(a, b, c, d, e, f) : fun(a)(b)(c)(d)(e)(f);
}
function A7(fun, a, b, c, d, e, f, g) {
  return fun.a === 7 ? fun.f(a, b, c, d, e, f, g) : fun(a)(b)(c)(d)(e)(f)(g);
}
function A8(fun, a, b, c, d, e, f, g, h) {
  return fun.a === 8 ? fun.f(a, b, c, d, e, f, g, h) : fun(a)(b)(c)(d)(e)(f)(g)(h);
}
function A9(fun, a, b, c, d, e, f, g, h, i) {
  return fun.a === 9 ? fun.f(a, b, c, d, e, f, g, h, i) : fun(a)(b)(c)(d)(e)(f)(g)(h)(i);
}




var _List_Nil = { $: 0 };
var _List_Nil_UNUSED = { $: '[]' };

function _List_Cons(hd, tl) { return { $: 1, a: hd, b: tl }; }
function _List_Cons_UNUSED(hd, tl) { return { $: '::', a: hd, b: tl }; }


var _List_cons = F2(_List_Cons);

function _List_fromArray(arr)
{
	var out = _List_Nil;
	for (var i = arr.length; i--; )
	{
		out = _List_Cons(arr[i], out);
	}
	return out;
}

function _List_toArray(xs)
{
	for (var out = []; xs.b; xs = xs.b) // WHILE_CONS
	{
		out.push(xs.a);
	}
	return out;
}

var _List_map2 = F3(function(f, xs, ys)
{
	for (var arr = []; xs.b && ys.b; xs = xs.b, ys = ys.b) // WHILE_CONSES
	{
		arr.push(A2(f, xs.a, ys.a));
	}
	return _List_fromArray(arr);
});

var _List_map3 = F4(function(f, xs, ys, zs)
{
	for (var arr = []; xs.b && ys.b && zs.b; xs = xs.b, ys = ys.b, zs = zs.b) // WHILE_CONSES
	{
		arr.push(A3(f, xs.a, ys.a, zs.a));
	}
	return _List_fromArray(arr);
});

var _List_map4 = F5(function(f, ws, xs, ys, zs)
{
	for (var arr = []; ws.b && xs.b && ys.b && zs.b; ws = ws.b, xs = xs.b, ys = ys.b, zs = zs.b) // WHILE_CONSES
	{
		arr.push(A4(f, ws.a, xs.a, ys.a, zs.a));
	}
	return _List_fromArray(arr);
});

var _List_map5 = F6(function(f, vs, ws, xs, ys, zs)
{
	for (var arr = []; vs.b && ws.b && xs.b && ys.b && zs.b; vs = vs.b, ws = ws.b, xs = xs.b, ys = ys.b, zs = zs.b) // WHILE_CONSES
	{
		arr.push(A5(f, vs.a, ws.a, xs.a, ys.a, zs.a));
	}
	return _List_fromArray(arr);
});

var _List_sortBy = F2(function(f, xs)
{
	return _List_fromArray(_List_toArray(xs).sort(function(a, b) {
		return _Utils_cmp(f(a), f(b));
	}));
});

var _List_sortWith = F2(function(f, xs)
{
	return _List_fromArray(_List_toArray(xs).sort(function(a, b) {
		var ord = A2(f, a, b);
		return ord === elm$core$Basics$EQ ? 0 : ord === elm$core$Basics$LT ? -1 : 1;
	}));
});



// EQUALITY

function _Utils_eq(x, y)
{
	for (
		var pair, stack = [], isEqual = _Utils_eqHelp(x, y, 0, stack);
		isEqual && (pair = stack.pop());
		isEqual = _Utils_eqHelp(pair.a, pair.b, 0, stack)
		)
	{}

	return isEqual;
}

function _Utils_eqHelp(x, y, depth, stack)
{
	if (depth > 100)
	{
		stack.push(_Utils_Tuple2(x,y));
		return true;
	}

	if (x === y)
	{
		return true;
	}

	if (typeof x !== 'object' || x === null || y === null)
	{
		typeof x === 'function' && _Debug_crash(5);
		return false;
	}

	/**_UNUSED/
	if (x.$ === 'Set_elm_builtin')
	{
		x = elm$core$Set$toList(x);
		y = elm$core$Set$toList(y);
	}
	if (x.$ === 'RBNode_elm_builtin' || x.$ === 'RBEmpty_elm_builtin')
	{
		x = elm$core$Dict$toList(x);
		y = elm$core$Dict$toList(y);
	}
	//*/

	/**/
	if (x.$ < 0)
	{
		x = elm$core$Dict$toList(x);
		y = elm$core$Dict$toList(y);
	}
	//*/

	for (var key in x)
	{
		if (!_Utils_eqHelp(x[key], y[key], depth + 1, stack))
		{
			return false;
		}
	}
	return true;
}

var _Utils_equal = F2(_Utils_eq);
var _Utils_notEqual = F2(function(a, b) { return !_Utils_eq(a,b); });



// COMPARISONS

// Code in Generate/JavaScript.hs, Basics.js, and List.js depends on
// the particular integer values assigned to LT, EQ, and GT.

function _Utils_cmp(x, y, ord)
{
	if (typeof x !== 'object')
	{
		return x === y ? /*EQ*/ 0 : x < y ? /*LT*/ -1 : /*GT*/ 1;
	}

	/**_UNUSED/
	if (x instanceof String)
	{
		var a = x.valueOf();
		var b = y.valueOf();
		return a === b ? 0 : a < b ? -1 : 1;
	}
	//*/

	/**/
	if (typeof x.$ === 'undefined')
	//*/
	/**_UNUSED/
	if (x.$[0] === '#')
	//*/
	{
		return (ord = _Utils_cmp(x.a, y.a))
			? ord
			: (ord = _Utils_cmp(x.b, y.b))
				? ord
				: _Utils_cmp(x.c, y.c);
	}

	// traverse conses until end of a list or a mismatch
	for (; x.b && y.b && !(ord = _Utils_cmp(x.a, y.a)); x = x.b, y = y.b) {} // WHILE_CONSES
	return ord || (x.b ? /*GT*/ 1 : y.b ? /*LT*/ -1 : /*EQ*/ 0);
}

var _Utils_lt = F2(function(a, b) { return _Utils_cmp(a, b) < 0; });
var _Utils_le = F2(function(a, b) { return _Utils_cmp(a, b) < 1; });
var _Utils_gt = F2(function(a, b) { return _Utils_cmp(a, b) > 0; });
var _Utils_ge = F2(function(a, b) { return _Utils_cmp(a, b) >= 0; });

var _Utils_compare = F2(function(x, y)
{
	var n = _Utils_cmp(x, y);
	return n < 0 ? elm$core$Basics$LT : n ? elm$core$Basics$GT : elm$core$Basics$EQ;
});


// COMMON VALUES

var _Utils_Tuple0 = 0;
var _Utils_Tuple0_UNUSED = { $: '#0' };

function _Utils_Tuple2(a, b) { return { a: a, b: b }; }
function _Utils_Tuple2_UNUSED(a, b) { return { $: '#2', a: a, b: b }; }

function _Utils_Tuple3(a, b, c) { return { a: a, b: b, c: c }; }
function _Utils_Tuple3_UNUSED(a, b, c) { return { $: '#3', a: a, b: b, c: c }; }

function _Utils_chr(c) { return c; }
function _Utils_chr_UNUSED(c) { return new String(c); }


// RECORDS

function _Utils_update(oldRecord, updatedFields)
{
	var newRecord = {};

	for (var key in oldRecord)
	{
		newRecord[key] = oldRecord[key];
	}

	for (var key in updatedFields)
	{
		newRecord[key] = updatedFields[key];
	}

	return newRecord;
}


// APPEND

var _Utils_append = F2(_Utils_ap);

function _Utils_ap(xs, ys)
{
	// append Strings
	if (typeof xs === 'string')
	{
		return xs + ys;
	}

	// append Lists
	if (!xs.b)
	{
		return ys;
	}
	var root = _List_Cons(xs.a, ys);
	xs = xs.b
	for (var curr = root; xs.b; xs = xs.b) // WHILE_CONS
	{
		curr = curr.b = _List_Cons(xs.a, ys);
	}
	return root;
}



var _JsArray_empty = [];

function _JsArray_singleton(value)
{
    return [value];
}

function _JsArray_length(array)
{
    return array.length;
}

var _JsArray_initialize = F3(function(size, offset, func)
{
    var result = new Array(size);

    for (var i = 0; i < size; i++)
    {
        result[i] = func(offset + i);
    }

    return result;
});

var _JsArray_initializeFromList = F2(function (max, ls)
{
    var result = new Array(max);

    for (var i = 0; i < max && ls.b; i++)
    {
        result[i] = ls.a;
        ls = ls.b;
    }

    result.length = i;
    return _Utils_Tuple2(result, ls);
});

var _JsArray_unsafeGet = F2(function(index, array)
{
    return array[index];
});

var _JsArray_unsafeSet = F3(function(index, value, array)
{
    var length = array.length;
    var result = new Array(length);

    for (var i = 0; i < length; i++)
    {
        result[i] = array[i];
    }

    result[index] = value;
    return result;
});

var _JsArray_push = F2(function(value, array)
{
    var length = array.length;
    var result = new Array(length + 1);

    for (var i = 0; i < length; i++)
    {
        result[i] = array[i];
    }

    result[length] = value;
    return result;
});

var _JsArray_foldl = F3(function(func, acc, array)
{
    var length = array.length;

    for (var i = 0; i < length; i++)
    {
        acc = A2(func, array[i], acc);
    }

    return acc;
});

var _JsArray_foldr = F3(function(func, acc, array)
{
    for (var i = array.length - 1; i >= 0; i--)
    {
        acc = A2(func, array[i], acc);
    }

    return acc;
});

var _JsArray_map = F2(function(func, array)
{
    var length = array.length;
    var result = new Array(length);

    for (var i = 0; i < length; i++)
    {
        result[i] = func(array[i]);
    }

    return result;
});

var _JsArray_indexedMap = F3(function(func, offset, array)
{
    var length = array.length;
    var result = new Array(length);

    for (var i = 0; i < length; i++)
    {
        result[i] = A2(func, offset + i, array[i]);
    }

    return result;
});

var _JsArray_slice = F3(function(from, to, array)
{
    return array.slice(from, to);
});

var _JsArray_appendN = F3(function(n, dest, source)
{
    var destLen = dest.length;
    var itemsToCopy = n - destLen;

    if (itemsToCopy > source.length)
    {
        itemsToCopy = source.length;
    }

    var size = destLen + itemsToCopy;
    var result = new Array(size);

    for (var i = 0; i < destLen; i++)
    {
        result[i] = dest[i];
    }

    for (var i = 0; i < itemsToCopy; i++)
    {
        result[i + destLen] = source[i];
    }

    return result;
});



// LOG

var _Debug_log = F2(function(tag, value)
{
	return value;
});

var _Debug_log_UNUSED = F2(function(tag, value)
{
	console.log(tag + ': ' + _Debug_toString(value));
	return value;
});


// TODOS

function _Debug_todo(moduleName, region)
{
	return function(message) {
		_Debug_crash(8, moduleName, region, message);
	};
}

function _Debug_todoCase(moduleName, region, value)
{
	return function(message) {
		_Debug_crash(9, moduleName, region, value, message);
	};
}


// TO STRING

function _Debug_toString(value)
{
	return '<internals>';
}

function _Debug_toString_UNUSED(value)
{
	return _Debug_toAnsiString(false, value);
}

function _Debug_toAnsiString(ansi, value)
{
	if (typeof value === 'function')
	{
		return _Debug_internalColor(ansi, '<function>');
	}

	if (typeof value === 'boolean')
	{
		return _Debug_ctorColor(ansi, value ? 'True' : 'False');
	}

	if (typeof value === 'number')
	{
		return _Debug_numberColor(ansi, value + '');
	}

	if (value instanceof String)
	{
		return _Debug_charColor(ansi, "'" + _Debug_addSlashes(value, true) + "'");
	}

	if (typeof value === 'string')
	{
		return _Debug_stringColor(ansi, '"' + _Debug_addSlashes(value, false) + '"');
	}

	if (typeof value === 'object' && '$' in value)
	{
		var tag = value.$;

		if (typeof tag === 'number')
		{
			return _Debug_internalColor(ansi, '<internals>');
		}

		if (tag[0] === '#')
		{
			var output = [];
			for (var k in value)
			{
				if (k === '$') continue;
				output.push(_Debug_toAnsiString(ansi, value[k]));
			}
			return '(' + output.join(',') + ')';
		}

		if (tag === 'Set_elm_builtin')
		{
			return _Debug_ctorColor(ansi, 'Set')
				+ _Debug_fadeColor(ansi, '.fromList') + ' '
				+ _Debug_toAnsiString(ansi, elm$core$Set$toList(value));
		}

		if (tag === 'RBNode_elm_builtin' || tag === 'RBEmpty_elm_builtin')
		{
			return _Debug_ctorColor(ansi, 'Dict')
				+ _Debug_fadeColor(ansi, '.fromList') + ' '
				+ _Debug_toAnsiString(ansi, elm$core$Dict$toList(value));
		}

		if (tag === 'Array_elm_builtin')
		{
			return _Debug_ctorColor(ansi, 'Array')
				+ _Debug_fadeColor(ansi, '.fromList') + ' '
				+ _Debug_toAnsiString(ansi, elm$core$Array$toList(value));
		}

		if (tag === '::' || tag === '[]')
		{
			var output = '[';

			value.b && (output += _Debug_toAnsiString(ansi, value.a), value = value.b)

			for (; value.b; value = value.b) // WHILE_CONS
			{
				output += ',' + _Debug_toAnsiString(ansi, value.a);
			}
			return output + ']';
		}

		var output = '';
		for (var i in value)
		{
			if (i === '$') continue;
			var str = _Debug_toAnsiString(ansi, value[i]);
			var c0 = str[0];
			var parenless = c0 === '{' || c0 === '(' || c0 === '[' || c0 === '<' || c0 === '"' || str.indexOf(' ') < 0;
			output += ' ' + (parenless ? str : '(' + str + ')');
		}
		return _Debug_ctorColor(ansi, tag) + output;
	}

	if (typeof DataView === 'function' && value instanceof DataView)
	{
		return _Debug_stringColor(ansi, '<' + value.byteLength + ' bytes>');
	}

	if (typeof File === 'function' && value instanceof File)
	{
		return _Debug_internalColor(ansi, '<' + value.name + '>');
	}

	if (typeof value === 'object')
	{
		var output = [];
		for (var key in value)
		{
			var field = key[0] === '_' ? key.slice(1) : key;
			output.push(_Debug_fadeColor(ansi, field) + ' = ' + _Debug_toAnsiString(ansi, value[key]));
		}
		if (output.length === 0)
		{
			return '{}';
		}
		return '{ ' + output.join(', ') + ' }';
	}

	return _Debug_internalColor(ansi, '<internals>');
}

function _Debug_addSlashes(str, isChar)
{
	var s = str
		.replace(/\\/g, '\\\\')
		.replace(/\n/g, '\\n')
		.replace(/\t/g, '\\t')
		.replace(/\r/g, '\\r')
		.replace(/\v/g, '\\v')
		.replace(/\0/g, '\\0');

	if (isChar)
	{
		return s.replace(/\'/g, '\\\'');
	}
	else
	{
		return s.replace(/\"/g, '\\"');
	}
}

function _Debug_ctorColor(ansi, string)
{
	return ansi ? '\x1b[96m' + string + '\x1b[0m' : string;
}

function _Debug_numberColor(ansi, string)
{
	return ansi ? '\x1b[95m' + string + '\x1b[0m' : string;
}

function _Debug_stringColor(ansi, string)
{
	return ansi ? '\x1b[93m' + string + '\x1b[0m' : string;
}

function _Debug_charColor(ansi, string)
{
	return ansi ? '\x1b[92m' + string + '\x1b[0m' : string;
}

function _Debug_fadeColor(ansi, string)
{
	return ansi ? '\x1b[37m' + string + '\x1b[0m' : string;
}

function _Debug_internalColor(ansi, string)
{
	return ansi ? '\x1b[94m' + string + '\x1b[0m' : string;
}

function _Debug_toHexDigit(n)
{
	return String.fromCharCode(n < 10 ? 48 + n : 55 + n);
}


// CRASH


function _Debug_crash(identifier)
{
	throw new Error('https://github.com/elm/core/blob/1.0.0/hints/' + identifier + '.md');
}


function _Debug_crash_UNUSED(identifier, fact1, fact2, fact3, fact4)
{
	switch(identifier)
	{
		case 0:
			throw new Error('What node should I take over? In JavaScript I need something like:\n\n    Elm.Main.init({\n        node: document.getElementById("elm-node")\n    })\n\nYou need to do this with any Browser.sandbox or Browser.element program.');

		case 1:
			throw new Error('Browser.application programs cannot handle URLs like this:\n\n    ' + document.location.href + '\n\nWhat is the root? The root of your file system? Try looking at this program with `elm reactor` or some other server.');

		case 2:
			var jsonErrorString = fact1;
			throw new Error('Problem with the flags given to your Elm program on initialization.\n\n' + jsonErrorString);

		case 3:
			var portName = fact1;
			throw new Error('There can only be one port named `' + portName + '`, but your program has multiple.');

		case 4:
			var portName = fact1;
			var problem = fact2;
			throw new Error('Trying to send an unexpected type of value through port `' + portName + '`:\n' + problem);

		case 5:
			throw new Error('Trying to use `(==)` on functions.\nThere is no way to know if functions are "the same" in the Elm sense.\nRead more about this at https://package.elm-lang.org/packages/elm/core/latest/Basics#== which describes why it is this way and what the better version will look like.');

		case 6:
			var moduleName = fact1;
			throw new Error('Your page is loading multiple Elm scripts with a module named ' + moduleName + '. Maybe a duplicate script is getting loaded accidentally? If not, rename one of them so I know which is which!');

		case 8:
			var moduleName = fact1;
			var region = fact2;
			var message = fact3;
			throw new Error('TODO in module `' + moduleName + '` ' + _Debug_regionToString(region) + '\n\n' + message);

		case 9:
			var moduleName = fact1;
			var region = fact2;
			var value = fact3;
			var message = fact4;
			throw new Error(
				'TODO in module `' + moduleName + '` from the `case` expression '
				+ _Debug_regionToString(region) + '\n\nIt received the following value:\n\n    '
				+ _Debug_toString(value).replace('\n', '\n    ')
				+ '\n\nBut the branch that handles it says:\n\n    ' + message.replace('\n', '\n    ')
			);

		case 10:
			throw new Error('Bug in https://github.com/elm/virtual-dom/issues');

		case 11:
			throw new Error('Cannot perform mod 0. Division by zero error.');
	}
}

function _Debug_regionToString(region)
{
	if (region.S.B === region.ah.B)
	{
		return 'on line ' + region.S.B;
	}
	return 'on lines ' + region.S.B + ' through ' + region.ah.B;
}



// MATH

var _Basics_add = F2(function(a, b) { return a + b; });
var _Basics_sub = F2(function(a, b) { return a - b; });
var _Basics_mul = F2(function(a, b) { return a * b; });
var _Basics_fdiv = F2(function(a, b) { return a / b; });
var _Basics_idiv = F2(function(a, b) { return (a / b) | 0; });
var _Basics_pow = F2(Math.pow);

var _Basics_remainderBy = F2(function(b, a) { return a % b; });

// https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/divmodnote-letter.pdf
var _Basics_modBy = F2(function(modulus, x)
{
	var answer = x % modulus;
	return modulus === 0
		? _Debug_crash(11)
		:
	((answer > 0 && modulus < 0) || (answer < 0 && modulus > 0))
		? answer + modulus
		: answer;
});


// TRIGONOMETRY

var _Basics_pi = Math.PI;
var _Basics_e = Math.E;
var _Basics_cos = Math.cos;
var _Basics_sin = Math.sin;
var _Basics_tan = Math.tan;
var _Basics_acos = Math.acos;
var _Basics_asin = Math.asin;
var _Basics_atan = Math.atan;
var _Basics_atan2 = F2(Math.atan2);


// MORE MATH

function _Basics_toFloat(x) { return x; }
function _Basics_truncate(n) { return n | 0; }
function _Basics_isInfinite(n) { return n === Infinity || n === -Infinity; }

var _Basics_ceiling = Math.ceil;
var _Basics_floor = Math.floor;
var _Basics_round = Math.round;
var _Basics_sqrt = Math.sqrt;
var _Basics_log = Math.log;
var _Basics_isNaN = isNaN;


// BOOLEANS

function _Basics_not(bool) { return !bool; }
var _Basics_and = F2(function(a, b) { return a && b; });
var _Basics_or  = F2(function(a, b) { return a || b; });
var _Basics_xor = F2(function(a, b) { return a !== b; });



function _Char_toCode(char)
{
	var code = char.charCodeAt(0);
	if (0xD800 <= code && code <= 0xDBFF)
	{
		return (code - 0xD800) * 0x400 + char.charCodeAt(1) - 0xDC00 + 0x10000
	}
	return code;
}

function _Char_fromCode(code)
{
	return _Utils_chr(
		(code < 0 || 0x10FFFF < code)
			? '\uFFFD'
			:
		(code <= 0xFFFF)
			? String.fromCharCode(code)
			:
		(code -= 0x10000,
			String.fromCharCode(Math.floor(code / 0x400) + 0xD800, code % 0x400 + 0xDC00)
		)
	);
}

function _Char_toUpper(char)
{
	return _Utils_chr(char.toUpperCase());
}

function _Char_toLower(char)
{
	return _Utils_chr(char.toLowerCase());
}

function _Char_toLocaleUpper(char)
{
	return _Utils_chr(char.toLocaleUpperCase());
}

function _Char_toLocaleLower(char)
{
	return _Utils_chr(char.toLocaleLowerCase());
}



var _String_cons = F2(function(chr, str)
{
	return chr + str;
});

function _String_uncons(string)
{
	var word = string.charCodeAt(0);
	return word
		? elm$core$Maybe$Just(
			0xD800 <= word && word <= 0xDBFF
				? _Utils_Tuple2(_Utils_chr(string[0] + string[1]), string.slice(2))
				: _Utils_Tuple2(_Utils_chr(string[0]), string.slice(1))
		)
		: elm$core$Maybe$Nothing;
}

var _String_append = F2(function(a, b)
{
	return a + b;
});

function _String_length(str)
{
	return str.length;
}

var _String_map = F2(function(func, string)
{
	var len = string.length;
	var array = new Array(len);
	var i = 0;
	while (i < len)
	{
		var word = string.charCodeAt(i);
		if (0xD800 <= word && word <= 0xDBFF)
		{
			array[i] = func(_Utils_chr(string[i] + string[i+1]));
			i += 2;
			continue;
		}
		array[i] = func(_Utils_chr(string[i]));
		i++;
	}
	return array.join('');
});

var _String_filter = F2(function(isGood, str)
{
	var arr = [];
	var len = str.length;
	var i = 0;
	while (i < len)
	{
		var char = str[i];
		var word = str.charCodeAt(i);
		i++;
		if (0xD800 <= word && word <= 0xDBFF)
		{
			char += str[i];
			i++;
		}

		if (isGood(_Utils_chr(char)))
		{
			arr.push(char);
		}
	}
	return arr.join('');
});

function _String_reverse(str)
{
	var len = str.length;
	var arr = new Array(len);
	var i = 0;
	while (i < len)
	{
		var word = str.charCodeAt(i);
		if (0xD800 <= word && word <= 0xDBFF)
		{
			arr[len - i] = str[i + 1];
			i++;
			arr[len - i] = str[i - 1];
			i++;
		}
		else
		{
			arr[len - i] = str[i];
			i++;
		}
	}
	return arr.join('');
}

var _String_foldl = F3(function(func, state, string)
{
	var len = string.length;
	var i = 0;
	while (i < len)
	{
		var char = string[i];
		var word = string.charCodeAt(i);
		i++;
		if (0xD800 <= word && word <= 0xDBFF)
		{
			char += string[i];
			i++;
		}
		state = A2(func, _Utils_chr(char), state);
	}
	return state;
});

var _String_foldr = F3(function(func, state, string)
{
	var i = string.length;
	while (i--)
	{
		var char = string[i];
		var word = string.charCodeAt(i);
		if (0xDC00 <= word && word <= 0xDFFF)
		{
			i--;
			char = string[i] + char;
		}
		state = A2(func, _Utils_chr(char), state);
	}
	return state;
});

var _String_split = F2(function(sep, str)
{
	return str.split(sep);
});

var _String_join = F2(function(sep, strs)
{
	return strs.join(sep);
});

var _String_slice = F3(function(start, end, str) {
	return str.slice(start, end);
});

function _String_trim(str)
{
	return str.trim();
}

function _String_trimLeft(str)
{
	return str.replace(/^\s+/, '');
}

function _String_trimRight(str)
{
	return str.replace(/\s+$/, '');
}

function _String_words(str)
{
	return _List_fromArray(str.trim().split(/\s+/g));
}

function _String_lines(str)
{
	return _List_fromArray(str.split(/\r\n|\r|\n/g));
}

function _String_toUpper(str)
{
	return str.toUpperCase();
}

function _String_toLower(str)
{
	return str.toLowerCase();
}

var _String_any = F2(function(isGood, string)
{
	var i = string.length;
	while (i--)
	{
		var char = string[i];
		var word = string.charCodeAt(i);
		if (0xDC00 <= word && word <= 0xDFFF)
		{
			i--;
			char = string[i] + char;
		}
		if (isGood(_Utils_chr(char)))
		{
			return true;
		}
	}
	return false;
});

var _String_all = F2(function(isGood, string)
{
	var i = string.length;
	while (i--)
	{
		var char = string[i];
		var word = string.charCodeAt(i);
		if (0xDC00 <= word && word <= 0xDFFF)
		{
			i--;
			char = string[i] + char;
		}
		if (!isGood(_Utils_chr(char)))
		{
			return false;
		}
	}
	return true;
});

var _String_contains = F2(function(sub, str)
{
	return str.indexOf(sub) > -1;
});

var _String_startsWith = F2(function(sub, str)
{
	return str.indexOf(sub) === 0;
});

var _String_endsWith = F2(function(sub, str)
{
	return str.length >= sub.length &&
		str.lastIndexOf(sub) === str.length - sub.length;
});

var _String_indexes = F2(function(sub, str)
{
	var subLen = sub.length;

	if (subLen < 1)
	{
		return _List_Nil;
	}

	var i = 0;
	var is = [];

	while ((i = str.indexOf(sub, i)) > -1)
	{
		is.push(i);
		i = i + subLen;
	}

	return _List_fromArray(is);
});


// TO STRING

function _String_fromNumber(number)
{
	return number + '';
}


// INT CONVERSIONS

function _String_toInt(str)
{
	var total = 0;
	var code0 = str.charCodeAt(0);
	var start = code0 == 0x2B /* + */ || code0 == 0x2D /* - */ ? 1 : 0;

	for (var i = start; i < str.length; ++i)
	{
		var code = str.charCodeAt(i);
		if (code < 0x30 || 0x39 < code)
		{
			return elm$core$Maybe$Nothing;
		}
		total = 10 * total + code - 0x30;
	}

	return i == start
		? elm$core$Maybe$Nothing
		: elm$core$Maybe$Just(code0 == 0x2D ? -total : total);
}


// FLOAT CONVERSIONS

function _String_toFloat(s)
{
	// check if it is a hex, octal, or binary number
	if (s.length === 0 || /[\sxbo]/.test(s))
	{
		return elm$core$Maybe$Nothing;
	}
	var n = +s;
	// faster isNaN check
	return n === n ? elm$core$Maybe$Just(n) : elm$core$Maybe$Nothing;
}

function _String_fromList(chars)
{
	return _List_toArray(chars).join('');
}




/**_UNUSED/
function _Json_errorToString(error)
{
	return elm$json$Json$Decode$errorToString(error);
}
//*/


// CORE DECODERS

function _Json_succeed(msg)
{
	return {
		$: 0,
		a: msg
	};
}

function _Json_fail(msg)
{
	return {
		$: 1,
		a: msg
	};
}

function _Json_decodePrim(decoder)
{
	return { $: 2, b: decoder };
}

var _Json_decodeInt = _Json_decodePrim(function(value) {
	return (typeof value !== 'number')
		? _Json_expecting('an INT', value)
		:
	(-2147483647 < value && value < 2147483647 && (value | 0) === value)
		? elm$core$Result$Ok(value)
		:
	(isFinite(value) && !(value % 1))
		? elm$core$Result$Ok(value)
		: _Json_expecting('an INT', value);
});

var _Json_decodeBool = _Json_decodePrim(function(value) {
	return (typeof value === 'boolean')
		? elm$core$Result$Ok(value)
		: _Json_expecting('a BOOL', value);
});

var _Json_decodeFloat = _Json_decodePrim(function(value) {
	return (typeof value === 'number')
		? elm$core$Result$Ok(value)
		: _Json_expecting('a FLOAT', value);
});

var _Json_decodeValue = _Json_decodePrim(function(value) {
	return elm$core$Result$Ok(_Json_wrap(value));
});

var _Json_decodeString = _Json_decodePrim(function(value) {
	return (typeof value === 'string')
		? elm$core$Result$Ok(value)
		: (value instanceof String)
			? elm$core$Result$Ok(value + '')
			: _Json_expecting('a STRING', value);
});

function _Json_decodeList(decoder) { return { $: 3, b: decoder }; }
function _Json_decodeArray(decoder) { return { $: 4, b: decoder }; }

function _Json_decodeNull(value) { return { $: 5, c: value }; }

var _Json_decodeField = F2(function(field, decoder)
{
	return {
		$: 6,
		d: field,
		b: decoder
	};
});

var _Json_decodeIndex = F2(function(index, decoder)
{
	return {
		$: 7,
		e: index,
		b: decoder
	};
});

function _Json_decodeKeyValuePairs(decoder)
{
	return {
		$: 8,
		b: decoder
	};
}

function _Json_mapMany(f, decoders)
{
	return {
		$: 9,
		f: f,
		g: decoders
	};
}

var _Json_andThen = F2(function(callback, decoder)
{
	return {
		$: 10,
		b: decoder,
		h: callback
	};
});

function _Json_oneOf(decoders)
{
	return {
		$: 11,
		g: decoders
	};
}


// DECODING OBJECTS

var _Json_map1 = F2(function(f, d1)
{
	return _Json_mapMany(f, [d1]);
});

var _Json_map2 = F3(function(f, d1, d2)
{
	return _Json_mapMany(f, [d1, d2]);
});

var _Json_map3 = F4(function(f, d1, d2, d3)
{
	return _Json_mapMany(f, [d1, d2, d3]);
});

var _Json_map4 = F5(function(f, d1, d2, d3, d4)
{
	return _Json_mapMany(f, [d1, d2, d3, d4]);
});

var _Json_map5 = F6(function(f, d1, d2, d3, d4, d5)
{
	return _Json_mapMany(f, [d1, d2, d3, d4, d5]);
});

var _Json_map6 = F7(function(f, d1, d2, d3, d4, d5, d6)
{
	return _Json_mapMany(f, [d1, d2, d3, d4, d5, d6]);
});

var _Json_map7 = F8(function(f, d1, d2, d3, d4, d5, d6, d7)
{
	return _Json_mapMany(f, [d1, d2, d3, d4, d5, d6, d7]);
});

var _Json_map8 = F9(function(f, d1, d2, d3, d4, d5, d6, d7, d8)
{
	return _Json_mapMany(f, [d1, d2, d3, d4, d5, d6, d7, d8]);
});


// DECODE

var _Json_runOnString = F2(function(decoder, string)
{
	try
	{
		var value = JSON.parse(string);
		return _Json_runHelp(decoder, value);
	}
	catch (e)
	{
		return elm$core$Result$Err(A2(elm$json$Json$Decode$Failure, 'This is not valid JSON! ' + e.message, _Json_wrap(string)));
	}
});

var _Json_run = F2(function(decoder, value)
{
	return _Json_runHelp(decoder, _Json_unwrap(value));
});

function _Json_runHelp(decoder, value)
{
	switch (decoder.$)
	{
		case 2:
			return decoder.b(value);

		case 5:
			return (value === null)
				? elm$core$Result$Ok(decoder.c)
				: _Json_expecting('null', value);

		case 3:
			if (!_Json_isArray(value))
			{
				return _Json_expecting('a LIST', value);
			}
			return _Json_runArrayDecoder(decoder.b, value, _List_fromArray);

		case 4:
			if (!_Json_isArray(value))
			{
				return _Json_expecting('an ARRAY', value);
			}
			return _Json_runArrayDecoder(decoder.b, value, _Json_toElmArray);

		case 6:
			var field = decoder.d;
			if (typeof value !== 'object' || value === null || !(field in value))
			{
				return _Json_expecting('an OBJECT with a field named `' + field + '`', value);
			}
			var result = _Json_runHelp(decoder.b, value[field]);
			return (elm$core$Result$isOk(result)) ? result : elm$core$Result$Err(A2(elm$json$Json$Decode$Field, field, result.a));

		case 7:
			var index = decoder.e;
			if (!_Json_isArray(value))
			{
				return _Json_expecting('an ARRAY', value);
			}
			if (index >= value.length)
			{
				return _Json_expecting('a LONGER array. Need index ' + index + ' but only see ' + value.length + ' entries', value);
			}
			var result = _Json_runHelp(decoder.b, value[index]);
			return (elm$core$Result$isOk(result)) ? result : elm$core$Result$Err(A2(elm$json$Json$Decode$Index, index, result.a));

		case 8:
			if (typeof value !== 'object' || value === null || _Json_isArray(value))
			{
				return _Json_expecting('an OBJECT', value);
			}

			var keyValuePairs = _List_Nil;
			// TODO test perf of Object.keys and switch when support is good enough
			for (var key in value)
			{
				if (value.hasOwnProperty(key))
				{
					var result = _Json_runHelp(decoder.b, value[key]);
					if (!elm$core$Result$isOk(result))
					{
						return elm$core$Result$Err(A2(elm$json$Json$Decode$Field, key, result.a));
					}
					keyValuePairs = _List_Cons(_Utils_Tuple2(key, result.a), keyValuePairs);
				}
			}
			return elm$core$Result$Ok(elm$core$List$reverse(keyValuePairs));

		case 9:
			var answer = decoder.f;
			var decoders = decoder.g;
			for (var i = 0; i < decoders.length; i++)
			{
				var result = _Json_runHelp(decoders[i], value);
				if (!elm$core$Result$isOk(result))
				{
					return result;
				}
				answer = answer(result.a);
			}
			return elm$core$Result$Ok(answer);

		case 10:
			var result = _Json_runHelp(decoder.b, value);
			return (!elm$core$Result$isOk(result))
				? result
				: _Json_runHelp(decoder.h(result.a), value);

		case 11:
			var errors = _List_Nil;
			for (var temp = decoder.g; temp.b; temp = temp.b) // WHILE_CONS
			{
				var result = _Json_runHelp(temp.a, value);
				if (elm$core$Result$isOk(result))
				{
					return result;
				}
				errors = _List_Cons(result.a, errors);
			}
			return elm$core$Result$Err(elm$json$Json$Decode$OneOf(elm$core$List$reverse(errors)));

		case 1:
			return elm$core$Result$Err(A2(elm$json$Json$Decode$Failure, decoder.a, _Json_wrap(value)));

		case 0:
			return elm$core$Result$Ok(decoder.a);
	}
}

function _Json_runArrayDecoder(decoder, value, toElmValue)
{
	var len = value.length;
	var array = new Array(len);
	for (var i = 0; i < len; i++)
	{
		var result = _Json_runHelp(decoder, value[i]);
		if (!elm$core$Result$isOk(result))
		{
			return elm$core$Result$Err(A2(elm$json$Json$Decode$Index, i, result.a));
		}
		array[i] = result.a;
	}
	return elm$core$Result$Ok(toElmValue(array));
}

function _Json_isArray(value)
{
	return Array.isArray(value) || (typeof FileList !== 'undefined' && value instanceof FileList);
}

function _Json_toElmArray(array)
{
	return A2(elm$core$Array$initialize, array.length, function(i) { return array[i]; });
}

function _Json_expecting(type, value)
{
	return elm$core$Result$Err(A2(elm$json$Json$Decode$Failure, 'Expecting ' + type, _Json_wrap(value)));
}


// EQUALITY

function _Json_equality(x, y)
{
	if (x === y)
	{
		return true;
	}

	if (x.$ !== y.$)
	{
		return false;
	}

	switch (x.$)
	{
		case 0:
		case 1:
			return x.a === y.a;

		case 2:
			return x.b === y.b;

		case 5:
			return x.c === y.c;

		case 3:
		case 4:
		case 8:
			return _Json_equality(x.b, y.b);

		case 6:
			return x.d === y.d && _Json_equality(x.b, y.b);

		case 7:
			return x.e === y.e && _Json_equality(x.b, y.b);

		case 9:
			return x.f === y.f && _Json_listEquality(x.g, y.g);

		case 10:
			return x.h === y.h && _Json_equality(x.b, y.b);

		case 11:
			return _Json_listEquality(x.g, y.g);
	}
}

function _Json_listEquality(aDecoders, bDecoders)
{
	var len = aDecoders.length;
	if (len !== bDecoders.length)
	{
		return false;
	}
	for (var i = 0; i < len; i++)
	{
		if (!_Json_equality(aDecoders[i], bDecoders[i]))
		{
			return false;
		}
	}
	return true;
}


// ENCODE

var _Json_encode = F2(function(indentLevel, value)
{
	return JSON.stringify(_Json_unwrap(value), null, indentLevel) + '';
});

function _Json_wrap_UNUSED(value) { return { $: 0, a: value }; }
function _Json_unwrap_UNUSED(value) { return value.a; }

function _Json_wrap(value) { return value; }
function _Json_unwrap(value) { return value; }

function _Json_emptyArray() { return []; }
function _Json_emptyObject() { return {}; }

var _Json_addField = F3(function(key, value, object)
{
	object[key] = _Json_unwrap(value);
	return object;
});

function _Json_addEntry(func)
{
	return F2(function(entry, array)
	{
		array.push(_Json_unwrap(func(entry)));
		return array;
	});
}

var _Json_encodeNull = _Json_wrap(null);



// TASKS

function _Scheduler_succeed(value)
{
	return {
		$: 0,
		a: value
	};
}

function _Scheduler_fail(error)
{
	return {
		$: 1,
		a: error
	};
}

function _Scheduler_binding(callback)
{
	return {
		$: 2,
		b: callback,
		c: null
	};
}

var _Scheduler_andThen = F2(function(callback, task)
{
	return {
		$: 3,
		b: callback,
		d: task
	};
});

var _Scheduler_onError = F2(function(callback, task)
{
	return {
		$: 4,
		b: callback,
		d: task
	};
});

function _Scheduler_receive(callback)
{
	return {
		$: 5,
		b: callback
	};
}


// PROCESSES

var _Scheduler_guid = 0;

function _Scheduler_rawSpawn(task)
{
	var proc = {
		$: 0,
		e: _Scheduler_guid++,
		f: task,
		g: null,
		h: []
	};

	_Scheduler_enqueue(proc);

	return proc;
}

function _Scheduler_spawn(task)
{
	return _Scheduler_binding(function(callback) {
		callback(_Scheduler_succeed(_Scheduler_rawSpawn(task)));
	});
}

function _Scheduler_rawSend(proc, msg)
{
	proc.h.push(msg);
	_Scheduler_enqueue(proc);
}

var _Scheduler_send = F2(function(proc, msg)
{
	return _Scheduler_binding(function(callback) {
		_Scheduler_rawSend(proc, msg);
		callback(_Scheduler_succeed(_Utils_Tuple0));
	});
});

function _Scheduler_kill(proc)
{
	return _Scheduler_binding(function(callback) {
		var task = proc.f;
		if (task.$ === 2 && task.c)
		{
			task.c();
		}

		proc.f = null;

		callback(_Scheduler_succeed(_Utils_Tuple0));
	});
}


/* STEP PROCESSES

type alias Process =
  { $ : tag
  , id : unique_id
  , root : Task
  , stack : null | { $: SUCCEED | FAIL, a: callback, b: stack }
  , mailbox : [msg]
  }

*/


var _Scheduler_working = false;
var _Scheduler_queue = [];


function _Scheduler_enqueue(proc)
{
	_Scheduler_queue.push(proc);
	if (_Scheduler_working)
	{
		return;
	}
	_Scheduler_working = true;
	while (proc = _Scheduler_queue.shift())
	{
		_Scheduler_step(proc);
	}
	_Scheduler_working = false;
}


function _Scheduler_step(proc)
{
	while (proc.f)
	{
		var rootTag = proc.f.$;
		if (rootTag === 0 || rootTag === 1)
		{
			while (proc.g && proc.g.$ !== rootTag)
			{
				proc.g = proc.g.i;
			}
			if (!proc.g)
			{
				return;
			}
			proc.f = proc.g.b(proc.f.a);
			proc.g = proc.g.i;
		}
		else if (rootTag === 2)
		{
			proc.f.c = proc.f.b(function(newRoot) {
				proc.f = newRoot;
				_Scheduler_enqueue(proc);
			});
			return;
		}
		else if (rootTag === 5)
		{
			if (proc.h.length === 0)
			{
				return;
			}
			proc.f = proc.f.b(proc.h.shift());
		}
		else // if (rootTag === 3 || rootTag === 4)
		{
			proc.g = {
				$: rootTag === 3 ? 0 : 1,
				b: proc.f.b,
				i: proc.g
			};
			proc.f = proc.f.d;
		}
	}
}



function _Process_sleep(time)
{
	return _Scheduler_binding(function(callback) {
		var id = setTimeout(function() {
			callback(_Scheduler_succeed(_Utils_Tuple0));
		}, time);

		return function() { clearTimeout(id); };
	});
}




// PROGRAMS


var _Platform_worker = F4(function(impl, flagDecoder, debugMetadata, args)
{
	return _Platform_initialize(
		flagDecoder,
		args,
		impl.a0,
		impl.bc,
		impl.bb,
		function() { return function() {} }
	);
});



// INITIALIZE A PROGRAM


function _Platform_initialize(flagDecoder, args, init, update, subscriptions, stepperBuilder)
{
	var result = A2(_Json_run, flagDecoder, _Json_wrap(args ? args['flags'] : undefined));
	elm$core$Result$isOk(result) || _Debug_crash(2 /**_UNUSED/, _Json_errorToString(result.a) /**/);
	var managers = {};
	result = init(result.a);
	var model = result.a;
	var stepper = stepperBuilder(sendToApp, model);
	var ports = _Platform_setupEffects(managers, sendToApp);

	function sendToApp(msg, viewMetadata)
	{
		result = A2(update, msg, model);
		stepper(model = result.a, viewMetadata);
		_Platform_dispatchEffects(managers, result.b, subscriptions(model));
	}

	_Platform_dispatchEffects(managers, result.b, subscriptions(model));

	return ports ? { ports: ports } : {};
}



// TRACK PRELOADS
//
// This is used by code in elm/browser and elm/http
// to register any HTTP requests that are triggered by init.
//


var _Platform_preload;


function _Platform_registerPreload(url)
{
	_Platform_preload.add(url);
}



// EFFECT MANAGERS


var _Platform_effectManagers = {};


function _Platform_setupEffects(managers, sendToApp)
{
	var ports;

	// setup all necessary effect managers
	for (var key in _Platform_effectManagers)
	{
		var manager = _Platform_effectManagers[key];

		if (manager.a)
		{
			ports = ports || {};
			ports[key] = manager.a(key, sendToApp);
		}

		managers[key] = _Platform_instantiateManager(manager, sendToApp);
	}

	return ports;
}


function _Platform_createManager(init, onEffects, onSelfMsg, cmdMap, subMap)
{
	return {
		b: init,
		c: onEffects,
		d: onSelfMsg,
		e: cmdMap,
		f: subMap
	};
}


function _Platform_instantiateManager(info, sendToApp)
{
	var router = {
		g: sendToApp,
		h: undefined
	};

	var onEffects = info.c;
	var onSelfMsg = info.d;
	var cmdMap = info.e;
	var subMap = info.f;

	function loop(state)
	{
		return A2(_Scheduler_andThen, loop, _Scheduler_receive(function(msg)
		{
			var value = msg.a;

			if (msg.$ === 0)
			{
				return A3(onSelfMsg, router, value, state);
			}

			return cmdMap && subMap
				? A4(onEffects, router, value.i, value.j, state)
				: A3(onEffects, router, cmdMap ? value.i : value.j, state);
		}));
	}

	return router.h = _Scheduler_rawSpawn(A2(_Scheduler_andThen, loop, info.b));
}



// ROUTING


var _Platform_sendToApp = F2(function(router, msg)
{
	return _Scheduler_binding(function(callback)
	{
		router.g(msg);
		callback(_Scheduler_succeed(_Utils_Tuple0));
	});
});


var _Platform_sendToSelf = F2(function(router, msg)
{
	return A2(_Scheduler_send, router.h, {
		$: 0,
		a: msg
	});
});



// BAGS


function _Platform_leaf(home)
{
	return function(value)
	{
		return {
			$: 1,
			k: home,
			l: value
		};
	};
}


function _Platform_batch(list)
{
	return {
		$: 2,
		m: list
	};
}


var _Platform_map = F2(function(tagger, bag)
{
	return {
		$: 3,
		n: tagger,
		o: bag
	}
});



// PIPE BAGS INTO EFFECT MANAGERS


function _Platform_dispatchEffects(managers, cmdBag, subBag)
{
	var effectsDict = {};
	_Platform_gatherEffects(true, cmdBag, effectsDict, null);
	_Platform_gatherEffects(false, subBag, effectsDict, null);

	for (var home in managers)
	{
		_Scheduler_rawSend(managers[home], {
			$: 'fx',
			a: effectsDict[home] || { i: _List_Nil, j: _List_Nil }
		});
	}
}


function _Platform_gatherEffects(isCmd, bag, effectsDict, taggers)
{
	switch (bag.$)
	{
		case 1:
			var home = bag.k;
			var effect = _Platform_toEffect(isCmd, home, taggers, bag.l);
			effectsDict[home] = _Platform_insert(isCmd, effect, effectsDict[home]);
			return;

		case 2:
			for (var list = bag.m; list.b; list = list.b) // WHILE_CONS
			{
				_Platform_gatherEffects(isCmd, list.a, effectsDict, taggers);
			}
			return;

		case 3:
			_Platform_gatherEffects(isCmd, bag.o, effectsDict, {
				p: bag.n,
				q: taggers
			});
			return;
	}
}


function _Platform_toEffect(isCmd, home, taggers, value)
{
	function applyTaggers(x)
	{
		for (var temp = taggers; temp; temp = temp.q)
		{
			x = temp.p(x);
		}
		return x;
	}

	var map = isCmd
		? _Platform_effectManagers[home].e
		: _Platform_effectManagers[home].f;

	return A2(map, applyTaggers, value)
}


function _Platform_insert(isCmd, newEffect, effects)
{
	effects = effects || { i: _List_Nil, j: _List_Nil };

	isCmd
		? (effects.i = _List_Cons(newEffect, effects.i))
		: (effects.j = _List_Cons(newEffect, effects.j));

	return effects;
}



// PORTS


function _Platform_checkPortName(name)
{
	if (_Platform_effectManagers[name])
	{
		_Debug_crash(3, name)
	}
}



// OUTGOING PORTS


function _Platform_outgoingPort(name, converter)
{
	_Platform_checkPortName(name);
	_Platform_effectManagers[name] = {
		e: _Platform_outgoingPortMap,
		r: converter,
		a: _Platform_setupOutgoingPort
	};
	return _Platform_leaf(name);
}


var _Platform_outgoingPortMap = F2(function(tagger, value) { return value; });


function _Platform_setupOutgoingPort(name)
{
	var subs = [];
	var converter = _Platform_effectManagers[name].r;

	// CREATE MANAGER

	var init = _Process_sleep(0);

	_Platform_effectManagers[name].b = init;
	_Platform_effectManagers[name].c = F3(function(router, cmdList, state)
	{
		for ( ; cmdList.b; cmdList = cmdList.b) // WHILE_CONS
		{
			// grab a separate reference to subs in case unsubscribe is called
			var currentSubs = subs;
			var value = _Json_unwrap(converter(cmdList.a));
			for (var i = 0; i < currentSubs.length; i++)
			{
				currentSubs[i](value);
			}
		}
		return init;
	});

	// PUBLIC API

	function subscribe(callback)
	{
		subs.push(callback);
	}

	function unsubscribe(callback)
	{
		// copy subs into a new array in case unsubscribe is called within a
		// subscribed callback
		subs = subs.slice();
		var index = subs.indexOf(callback);
		if (index >= 0)
		{
			subs.splice(index, 1);
		}
	}

	return {
		subscribe: subscribe,
		unsubscribe: unsubscribe
	};
}



// INCOMING PORTS


function _Platform_incomingPort(name, converter)
{
	_Platform_checkPortName(name);
	_Platform_effectManagers[name] = {
		f: _Platform_incomingPortMap,
		r: converter,
		a: _Platform_setupIncomingPort
	};
	return _Platform_leaf(name);
}


var _Platform_incomingPortMap = F2(function(tagger, finalTagger)
{
	return function(value)
	{
		return tagger(finalTagger(value));
	};
});


function _Platform_setupIncomingPort(name, sendToApp)
{
	var subs = _List_Nil;
	var converter = _Platform_effectManagers[name].r;

	// CREATE MANAGER

	var init = _Scheduler_succeed(null);

	_Platform_effectManagers[name].b = init;
	_Platform_effectManagers[name].c = F3(function(router, subList, state)
	{
		subs = subList;
		return init;
	});

	// PUBLIC API

	function send(incomingValue)
	{
		var result = A2(_Json_run, converter, _Json_wrap(incomingValue));

		elm$core$Result$isOk(result) || _Debug_crash(4, name, result.a);

		var value = result.a;
		for (var temp = subs; temp.b; temp = temp.b) // WHILE_CONS
		{
			sendToApp(temp.a(value));
		}
	}

	return { send: send };
}



// EXPORT ELM MODULES
//
// Have DEBUG and PROD versions so that we can (1) give nicer errors in
// debug mode and (2) not pay for the bits needed for that in prod mode.
//


function _Platform_export(exports)
{
	scope['Elm']
		? _Platform_mergeExportsProd(scope['Elm'], exports)
		: scope['Elm'] = exports;
}


function _Platform_mergeExportsProd(obj, exports)
{
	for (var name in exports)
	{
		(name in obj)
			? (name == 'init')
				? _Debug_crash(6)
				: _Platform_mergeExportsProd(obj[name], exports[name])
			: (obj[name] = exports[name]);
	}
}


function _Platform_export_UNUSED(exports)
{
	scope['Elm']
		? _Platform_mergeExportsDebug('Elm', scope['Elm'], exports)
		: scope['Elm'] = exports;
}


function _Platform_mergeExportsDebug(moduleName, obj, exports)
{
	for (var name in exports)
	{
		(name in obj)
			? (name == 'init')
				? _Debug_crash(6, moduleName)
				: _Platform_mergeExportsDebug(moduleName + '.' + name, obj[name], exports[name])
			: (obj[name] = exports[name]);
	}
}




// HELPERS


var _VirtualDom_divertHrefToApp;

var _VirtualDom_doc = typeof document !== 'undefined' ? document : {};


function _VirtualDom_appendChild(parent, child)
{
	parent.appendChild(child);
}

var _VirtualDom_init = F4(function(virtualNode, flagDecoder, debugMetadata, args)
{
	// NOTE: this function needs _Platform_export available to work

	/**/
	var node = args['node'];
	//*/
	/**_UNUSED/
	var node = args && args['node'] ? args['node'] : _Debug_crash(0);
	//*/

	node.parentNode.replaceChild(
		_VirtualDom_render(virtualNode, function() {}),
		node
	);

	return {};
});



// TEXT


function _VirtualDom_text(string)
{
	return {
		$: 0,
		a: string
	};
}



// NODE


var _VirtualDom_nodeNS = F2(function(namespace, tag)
{
	return F2(function(factList, kidList)
	{
		for (var kids = [], descendantsCount = 0; kidList.b; kidList = kidList.b) // WHILE_CONS
		{
			var kid = kidList.a;
			descendantsCount += (kid.b || 0);
			kids.push(kid);
		}
		descendantsCount += kids.length;

		return {
			$: 1,
			c: tag,
			d: _VirtualDom_organizeFacts(factList),
			e: kids,
			f: namespace,
			b: descendantsCount
		};
	});
});


var _VirtualDom_node = _VirtualDom_nodeNS(undefined);



// KEYED NODE


var _VirtualDom_keyedNodeNS = F2(function(namespace, tag)
{
	return F2(function(factList, kidList)
	{
		for (var kids = [], descendantsCount = 0; kidList.b; kidList = kidList.b) // WHILE_CONS
		{
			var kid = kidList.a;
			descendantsCount += (kid.b.b || 0);
			kids.push(kid);
		}
		descendantsCount += kids.length;

		return {
			$: 2,
			c: tag,
			d: _VirtualDom_organizeFacts(factList),
			e: kids,
			f: namespace,
			b: descendantsCount
		};
	});
});


var _VirtualDom_keyedNode = _VirtualDom_keyedNodeNS(undefined);



// CUSTOM


function _VirtualDom_custom(factList, model, render, diff)
{
	return {
		$: 3,
		d: _VirtualDom_organizeFacts(factList),
		g: model,
		h: render,
		i: diff
	};
}



// MAP


var _VirtualDom_map = F2(function(tagger, node)
{
	return {
		$: 4,
		j: tagger,
		k: node,
		b: 1 + (node.b || 0)
	};
});



// LAZY


function _VirtualDom_thunk(refs, thunk)
{
	return {
		$: 5,
		l: refs,
		m: thunk,
		k: undefined
	};
}

var _VirtualDom_lazy = F2(function(func, a)
{
	return _VirtualDom_thunk([func, a], function() {
		return func(a);
	});
});

var _VirtualDom_lazy2 = F3(function(func, a, b)
{
	return _VirtualDom_thunk([func, a, b], function() {
		return A2(func, a, b);
	});
});

var _VirtualDom_lazy3 = F4(function(func, a, b, c)
{
	return _VirtualDom_thunk([func, a, b, c], function() {
		return A3(func, a, b, c);
	});
});

var _VirtualDom_lazy4 = F5(function(func, a, b, c, d)
{
	return _VirtualDom_thunk([func, a, b, c, d], function() {
		return A4(func, a, b, c, d);
	});
});

var _VirtualDom_lazy5 = F6(function(func, a, b, c, d, e)
{
	return _VirtualDom_thunk([func, a, b, c, d, e], function() {
		return A5(func, a, b, c, d, e);
	});
});

var _VirtualDom_lazy6 = F7(function(func, a, b, c, d, e, f)
{
	return _VirtualDom_thunk([func, a, b, c, d, e, f], function() {
		return A6(func, a, b, c, d, e, f);
	});
});

var _VirtualDom_lazy7 = F8(function(func, a, b, c, d, e, f, g)
{
	return _VirtualDom_thunk([func, a, b, c, d, e, f, g], function() {
		return A7(func, a, b, c, d, e, f, g);
	});
});

var _VirtualDom_lazy8 = F9(function(func, a, b, c, d, e, f, g, h)
{
	return _VirtualDom_thunk([func, a, b, c, d, e, f, g, h], function() {
		return A8(func, a, b, c, d, e, f, g, h);
	});
});



// FACTS


var _VirtualDom_on = F2(function(key, handler)
{
	return {
		$: 'a0',
		n: key,
		o: handler
	};
});
var _VirtualDom_style = F2(function(key, value)
{
	return {
		$: 'a1',
		n: key,
		o: value
	};
});
var _VirtualDom_property = F2(function(key, value)
{
	return {
		$: 'a2',
		n: key,
		o: value
	};
});
var _VirtualDom_attribute = F2(function(key, value)
{
	return {
		$: 'a3',
		n: key,
		o: value
	};
});
var _VirtualDom_attributeNS = F3(function(namespace, key, value)
{
	return {
		$: 'a4',
		n: key,
		o: { f: namespace, o: value }
	};
});



// XSS ATTACK VECTOR CHECKS


function _VirtualDom_noScript(tag)
{
	return tag == 'script' ? 'p' : tag;
}

function _VirtualDom_noOnOrFormAction(key)
{
	return /^(on|formAction$)/i.test(key) ? 'data-' + key : key;
}

function _VirtualDom_noInnerHtmlOrFormAction(key)
{
	return key == 'innerHTML' || key == 'formAction' ? 'data-' + key : key;
}

function _VirtualDom_noJavaScriptUri(value)
{
	return /^javascript:/i.test(value.replace(/\s/g,'')) ? '' : value;
}

function _VirtualDom_noJavaScriptUri_UNUSED(value)
{
	return /^javascript:/i.test(value.replace(/\s/g,''))
		? 'javascript:alert("This is an XSS vector. Please use ports or web components instead.")'
		: value;
}

function _VirtualDom_noJavaScriptOrHtmlUri(value)
{
	return /^\s*(javascript:|data:text\/html)/i.test(value) ? '' : value;
}

function _VirtualDom_noJavaScriptOrHtmlUri_UNUSED(value)
{
	return /^\s*(javascript:|data:text\/html)/i.test(value)
		? 'javascript:alert("This is an XSS vector. Please use ports or web components instead.")'
		: value;
}



// MAP FACTS


var _VirtualDom_mapAttribute = F2(function(func, attr)
{
	return (attr.$ === 'a0')
		? A2(_VirtualDom_on, attr.n, _VirtualDom_mapHandler(func, attr.o))
		: attr;
});

function _VirtualDom_mapHandler(func, handler)
{
	var tag = elm$virtual_dom$VirtualDom$toHandlerInt(handler);

	// 0 = Normal
	// 1 = MayStopPropagation
	// 2 = MayPreventDefault
	// 3 = Custom

	return {
		$: handler.$,
		a:
			!tag
				? A2(elm$json$Json$Decode$map, func, handler.a)
				:
			A3(elm$json$Json$Decode$map2,
				tag < 3
					? _VirtualDom_mapEventTuple
					: _VirtualDom_mapEventRecord,
				elm$json$Json$Decode$succeed(func),
				handler.a
			)
	};
}

var _VirtualDom_mapEventTuple = F2(function(func, tuple)
{
	return _Utils_Tuple2(func(tuple.a), tuple.b);
});

var _VirtualDom_mapEventRecord = F2(function(func, record)
{
	return {
		a1: func(record.a1),
		a9: record.a9,
		a5: record.a5
	}
});



// ORGANIZE FACTS


function _VirtualDom_organizeFacts(factList)
{
	for (var facts = {}; factList.b; factList = factList.b) // WHILE_CONS
	{
		var entry = factList.a;

		var tag = entry.$;
		var key = entry.n;
		var value = entry.o;

		if (tag === 'a2')
		{
			(key === 'className')
				? _VirtualDom_addClass(facts, key, _Json_unwrap(value))
				: facts[key] = _Json_unwrap(value);

			continue;
		}

		var subFacts = facts[tag] || (facts[tag] = {});
		(tag === 'a3' && key === 'class')
			? _VirtualDom_addClass(subFacts, key, value)
			: subFacts[key] = value;
	}

	return facts;
}

function _VirtualDom_addClass(object, key, newClass)
{
	var classes = object[key];
	object[key] = classes ? classes + ' ' + newClass : newClass;
}



// RENDER


function _VirtualDom_render(vNode, eventNode)
{
	var tag = vNode.$;

	if (tag === 5)
	{
		return _VirtualDom_render(vNode.k || (vNode.k = vNode.m()), eventNode);
	}

	if (tag === 0)
	{
		return _VirtualDom_doc.createTextNode(vNode.a);
	}

	if (tag === 4)
	{
		var subNode = vNode.k;
		var tagger = vNode.j;

		while (subNode.$ === 4)
		{
			typeof tagger !== 'object'
				? tagger = [tagger, subNode.j]
				: tagger.push(subNode.j);

			subNode = subNode.k;
		}

		var subEventRoot = { j: tagger, p: eventNode };
		var domNode = _VirtualDom_render(subNode, subEventRoot);
		domNode.elm_event_node_ref = subEventRoot;
		return domNode;
	}

	if (tag === 3)
	{
		var domNode = vNode.h(vNode.g);
		_VirtualDom_applyFacts(domNode, eventNode, vNode.d);
		return domNode;
	}

	// at this point `tag` must be 1 or 2

	var domNode = vNode.f
		? _VirtualDom_doc.createElementNS(vNode.f, vNode.c)
		: _VirtualDom_doc.createElement(vNode.c);

	if (_VirtualDom_divertHrefToApp && vNode.c == 'a')
	{
		domNode.addEventListener('click', _VirtualDom_divertHrefToApp(domNode));
	}

	_VirtualDom_applyFacts(domNode, eventNode, vNode.d);

	for (var kids = vNode.e, i = 0; i < kids.length; i++)
	{
		_VirtualDom_appendChild(domNode, _VirtualDom_render(tag === 1 ? kids[i] : kids[i].b, eventNode));
	}

	return domNode;
}



// APPLY FACTS


function _VirtualDom_applyFacts(domNode, eventNode, facts)
{
	for (var key in facts)
	{
		var value = facts[key];

		key === 'a1'
			? _VirtualDom_applyStyles(domNode, value)
			:
		key === 'a0'
			? _VirtualDom_applyEvents(domNode, eventNode, value)
			:
		key === 'a3'
			? _VirtualDom_applyAttrs(domNode, value)
			:
		key === 'a4'
			? _VirtualDom_applyAttrsNS(domNode, value)
			:
		((key !== 'value' && key !== 'checked') || domNode[key] !== value) && (domNode[key] = value);
	}
}



// APPLY STYLES


function _VirtualDom_applyStyles(domNode, styles)
{
	var domNodeStyle = domNode.style;

	for (var key in styles)
	{
		domNodeStyle[key] = styles[key];
	}
}



// APPLY ATTRS


function _VirtualDom_applyAttrs(domNode, attrs)
{
	for (var key in attrs)
	{
		var value = attrs[key];
		typeof value !== 'undefined'
			? domNode.setAttribute(key, value)
			: domNode.removeAttribute(key);
	}
}



// APPLY NAMESPACED ATTRS


function _VirtualDom_applyAttrsNS(domNode, nsAttrs)
{
	for (var key in nsAttrs)
	{
		var pair = nsAttrs[key];
		var namespace = pair.f;
		var value = pair.o;

		typeof value !== 'undefined'
			? domNode.setAttributeNS(namespace, key, value)
			: domNode.removeAttributeNS(namespace, key);
	}
}



// APPLY EVENTS


function _VirtualDom_applyEvents(domNode, eventNode, events)
{
	var allCallbacks = domNode.elmFs || (domNode.elmFs = {});

	for (var key in events)
	{
		var newHandler = events[key];
		var oldCallback = allCallbacks[key];

		if (!newHandler)
		{
			domNode.removeEventListener(key, oldCallback);
			allCallbacks[key] = undefined;
			continue;
		}

		if (oldCallback)
		{
			var oldHandler = oldCallback.q;
			if (oldHandler.$ === newHandler.$)
			{
				oldCallback.q = newHandler;
				continue;
			}
			domNode.removeEventListener(key, oldCallback);
		}

		oldCallback = _VirtualDom_makeCallback(eventNode, newHandler);
		domNode.addEventListener(key, oldCallback,
			_VirtualDom_passiveSupported
			&& { passive: elm$virtual_dom$VirtualDom$toHandlerInt(newHandler) < 2 }
		);
		allCallbacks[key] = oldCallback;
	}
}



// PASSIVE EVENTS


var _VirtualDom_passiveSupported;

try
{
	window.addEventListener('t', null, Object.defineProperty({}, 'passive', {
		get: function() { _VirtualDom_passiveSupported = true; }
	}));
}
catch(e) {}



// EVENT HANDLERS


function _VirtualDom_makeCallback(eventNode, initialHandler)
{
	function callback(event)
	{
		var handler = callback.q;
		var result = _Json_runHelp(handler.a, event);

		if (!elm$core$Result$isOk(result))
		{
			return;
		}

		var tag = elm$virtual_dom$VirtualDom$toHandlerInt(handler);

		// 0 = Normal
		// 1 = MayStopPropagation
		// 2 = MayPreventDefault
		// 3 = Custom

		var value = result.a;
		var message = !tag ? value : tag < 3 ? value.a : value.a1;
		var stopPropagation = tag == 1 ? value.b : tag == 3 && value.a9;
		var currentEventNode = (
			stopPropagation && event.stopPropagation(),
			(tag == 2 ? value.b : tag == 3 && value.a5) && event.preventDefault(),
			eventNode
		);
		var tagger;
		var i;
		while (tagger = currentEventNode.j)
		{
			if (typeof tagger == 'function')
			{
				message = tagger(message);
			}
			else
			{
				for (var i = tagger.length; i--; )
				{
					message = tagger[i](message);
				}
			}
			currentEventNode = currentEventNode.p;
		}
		currentEventNode(message, stopPropagation); // stopPropagation implies isSync
	}

	callback.q = initialHandler;

	return callback;
}

function _VirtualDom_equalEvents(x, y)
{
	return x.$ == y.$ && _Json_equality(x.a, y.a);
}



// DIFF


// TODO: Should we do patches like in iOS?
//
// type Patch
//   = At Int Patch
//   | Batch (List Patch)
//   | Change ...
//
// How could it not be better?
//
function _VirtualDom_diff(x, y)
{
	var patches = [];
	_VirtualDom_diffHelp(x, y, patches, 0);
	return patches;
}


function _VirtualDom_pushPatch(patches, type, index, data)
{
	var patch = {
		$: type,
		r: index,
		s: data,
		t: undefined,
		u: undefined
	};
	patches.push(patch);
	return patch;
}


function _VirtualDom_diffHelp(x, y, patches, index)
{
	if (x === y)
	{
		return;
	}

	var xType = x.$;
	var yType = y.$;

	// Bail if you run into different types of nodes. Implies that the
	// structure has changed significantly and it's not worth a diff.
	if (xType !== yType)
	{
		if (xType === 1 && yType === 2)
		{
			y = _VirtualDom_dekey(y);
			yType = 1;
		}
		else
		{
			_VirtualDom_pushPatch(patches, 0, index, y);
			return;
		}
	}

	// Now we know that both nodes are the same $.
	switch (yType)
	{
		case 5:
			var xRefs = x.l;
			var yRefs = y.l;
			var i = xRefs.length;
			var same = i === yRefs.length;
			while (same && i--)
			{
				same = xRefs[i] === yRefs[i];
			}
			if (same)
			{
				y.k = x.k;
				return;
			}
			y.k = y.m();
			var subPatches = [];
			_VirtualDom_diffHelp(x.k, y.k, subPatches, 0);
			subPatches.length > 0 && _VirtualDom_pushPatch(patches, 1, index, subPatches);
			return;

		case 4:
			// gather nested taggers
			var xTaggers = x.j;
			var yTaggers = y.j;
			var nesting = false;

			var xSubNode = x.k;
			while (xSubNode.$ === 4)
			{
				nesting = true;

				typeof xTaggers !== 'object'
					? xTaggers = [xTaggers, xSubNode.j]
					: xTaggers.push(xSubNode.j);

				xSubNode = xSubNode.k;
			}

			var ySubNode = y.k;
			while (ySubNode.$ === 4)
			{
				nesting = true;

				typeof yTaggers !== 'object'
					? yTaggers = [yTaggers, ySubNode.j]
					: yTaggers.push(ySubNode.j);

				ySubNode = ySubNode.k;
			}

			// Just bail if different numbers of taggers. This implies the
			// structure of the virtual DOM has changed.
			if (nesting && xTaggers.length !== yTaggers.length)
			{
				_VirtualDom_pushPatch(patches, 0, index, y);
				return;
			}

			// check if taggers are "the same"
			if (nesting ? !_VirtualDom_pairwiseRefEqual(xTaggers, yTaggers) : xTaggers !== yTaggers)
			{
				_VirtualDom_pushPatch(patches, 2, index, yTaggers);
			}

			// diff everything below the taggers
			_VirtualDom_diffHelp(xSubNode, ySubNode, patches, index + 1);
			return;

		case 0:
			if (x.a !== y.a)
			{
				_VirtualDom_pushPatch(patches, 3, index, y.a);
			}
			return;

		case 1:
			_VirtualDom_diffNodes(x, y, patches, index, _VirtualDom_diffKids);
			return;

		case 2:
			_VirtualDom_diffNodes(x, y, patches, index, _VirtualDom_diffKeyedKids);
			return;

		case 3:
			if (x.h !== y.h)
			{
				_VirtualDom_pushPatch(patches, 0, index, y);
				return;
			}

			var factsDiff = _VirtualDom_diffFacts(x.d, y.d);
			factsDiff && _VirtualDom_pushPatch(patches, 4, index, factsDiff);

			var patch = y.i(x.g, y.g);
			patch && _VirtualDom_pushPatch(patches, 5, index, patch);

			return;
	}
}

// assumes the incoming arrays are the same length
function _VirtualDom_pairwiseRefEqual(as, bs)
{
	for (var i = 0; i < as.length; i++)
	{
		if (as[i] !== bs[i])
		{
			return false;
		}
	}

	return true;
}

function _VirtualDom_diffNodes(x, y, patches, index, diffKids)
{
	// Bail if obvious indicators have changed. Implies more serious
	// structural changes such that it's not worth it to diff.
	if (x.c !== y.c || x.f !== y.f)
	{
		_VirtualDom_pushPatch(patches, 0, index, y);
		return;
	}

	var factsDiff = _VirtualDom_diffFacts(x.d, y.d);
	factsDiff && _VirtualDom_pushPatch(patches, 4, index, factsDiff);

	diffKids(x, y, patches, index);
}



// DIFF FACTS


// TODO Instead of creating a new diff object, it's possible to just test if
// there *is* a diff. During the actual patch, do the diff again and make the
// modifications directly. This way, there's no new allocations. Worth it?
function _VirtualDom_diffFacts(x, y, category)
{
	var diff;

	// look for changes and removals
	for (var xKey in x)
	{
		if (xKey === 'a1' || xKey === 'a0' || xKey === 'a3' || xKey === 'a4')
		{
			var subDiff = _VirtualDom_diffFacts(x[xKey], y[xKey] || {}, xKey);
			if (subDiff)
			{
				diff = diff || {};
				diff[xKey] = subDiff;
			}
			continue;
		}

		// remove if not in the new facts
		if (!(xKey in y))
		{
			diff = diff || {};
			diff[xKey] =
				!category
					? (typeof x[xKey] === 'string' ? '' : null)
					:
				(category === 'a1')
					? ''
					:
				(category === 'a0' || category === 'a3')
					? undefined
					:
				{ f: x[xKey].f, o: undefined };

			continue;
		}

		var xValue = x[xKey];
		var yValue = y[xKey];

		// reference equal, so don't worry about it
		if (xValue === yValue && xKey !== 'value' && xKey !== 'checked'
			|| category === 'a0' && _VirtualDom_equalEvents(xValue, yValue))
		{
			continue;
		}

		diff = diff || {};
		diff[xKey] = yValue;
	}

	// add new stuff
	for (var yKey in y)
	{
		if (!(yKey in x))
		{
			diff = diff || {};
			diff[yKey] = y[yKey];
		}
	}

	return diff;
}



// DIFF KIDS


function _VirtualDom_diffKids(xParent, yParent, patches, index)
{
	var xKids = xParent.e;
	var yKids = yParent.e;

	var xLen = xKids.length;
	var yLen = yKids.length;

	// FIGURE OUT IF THERE ARE INSERTS OR REMOVALS

	if (xLen > yLen)
	{
		_VirtualDom_pushPatch(patches, 6, index, {
			v: yLen,
			i: xLen - yLen
		});
	}
	else if (xLen < yLen)
	{
		_VirtualDom_pushPatch(patches, 7, index, {
			v: xLen,
			e: yKids
		});
	}

	// PAIRWISE DIFF EVERYTHING ELSE

	for (var minLen = xLen < yLen ? xLen : yLen, i = 0; i < minLen; i++)
	{
		var xKid = xKids[i];
		_VirtualDom_diffHelp(xKid, yKids[i], patches, ++index);
		index += xKid.b || 0;
	}
}



// KEYED DIFF


function _VirtualDom_diffKeyedKids(xParent, yParent, patches, rootIndex)
{
	var localPatches = [];

	var changes = {}; // Dict String Entry
	var inserts = []; // Array { index : Int, entry : Entry }
	// type Entry = { tag : String, vnode : VNode, index : Int, data : _ }

	var xKids = xParent.e;
	var yKids = yParent.e;
	var xLen = xKids.length;
	var yLen = yKids.length;
	var xIndex = 0;
	var yIndex = 0;

	var index = rootIndex;

	while (xIndex < xLen && yIndex < yLen)
	{
		var x = xKids[xIndex];
		var y = yKids[yIndex];

		var xKey = x.a;
		var yKey = y.a;
		var xNode = x.b;
		var yNode = y.b;

		var newMatch = undefined;
		var oldMatch = undefined;

		// check if keys match

		if (xKey === yKey)
		{
			index++;
			_VirtualDom_diffHelp(xNode, yNode, localPatches, index);
			index += xNode.b || 0;

			xIndex++;
			yIndex++;
			continue;
		}

		// look ahead 1 to detect insertions and removals.

		var xNext = xKids[xIndex + 1];
		var yNext = yKids[yIndex + 1];

		if (xNext)
		{
			var xNextKey = xNext.a;
			var xNextNode = xNext.b;
			oldMatch = yKey === xNextKey;
		}

		if (yNext)
		{
			var yNextKey = yNext.a;
			var yNextNode = yNext.b;
			newMatch = xKey === yNextKey;
		}


		// swap x and y
		if (newMatch && oldMatch)
		{
			index++;
			_VirtualDom_diffHelp(xNode, yNextNode, localPatches, index);
			_VirtualDom_insertNode(changes, localPatches, xKey, yNode, yIndex, inserts);
			index += xNode.b || 0;

			index++;
			_VirtualDom_removeNode(changes, localPatches, xKey, xNextNode, index);
			index += xNextNode.b || 0;

			xIndex += 2;
			yIndex += 2;
			continue;
		}

		// insert y
		if (newMatch)
		{
			index++;
			_VirtualDom_insertNode(changes, localPatches, yKey, yNode, yIndex, inserts);
			_VirtualDom_diffHelp(xNode, yNextNode, localPatches, index);
			index += xNode.b || 0;

			xIndex += 1;
			yIndex += 2;
			continue;
		}

		// remove x
		if (oldMatch)
		{
			index++;
			_VirtualDom_removeNode(changes, localPatches, xKey, xNode, index);
			index += xNode.b || 0;

			index++;
			_VirtualDom_diffHelp(xNextNode, yNode, localPatches, index);
			index += xNextNode.b || 0;

			xIndex += 2;
			yIndex += 1;
			continue;
		}

		// remove x, insert y
		if (xNext && xNextKey === yNextKey)
		{
			index++;
			_VirtualDom_removeNode(changes, localPatches, xKey, xNode, index);
			_VirtualDom_insertNode(changes, localPatches, yKey, yNode, yIndex, inserts);
			index += xNode.b || 0;

			index++;
			_VirtualDom_diffHelp(xNextNode, yNextNode, localPatches, index);
			index += xNextNode.b || 0;

			xIndex += 2;
			yIndex += 2;
			continue;
		}

		break;
	}

	// eat up any remaining nodes with removeNode and insertNode

	while (xIndex < xLen)
	{
		index++;
		var x = xKids[xIndex];
		var xNode = x.b;
		_VirtualDom_removeNode(changes, localPatches, x.a, xNode, index);
		index += xNode.b || 0;
		xIndex++;
	}

	while (yIndex < yLen)
	{
		var endInserts = endInserts || [];
		var y = yKids[yIndex];
		_VirtualDom_insertNode(changes, localPatches, y.a, y.b, undefined, endInserts);
		yIndex++;
	}

	if (localPatches.length > 0 || inserts.length > 0 || endInserts)
	{
		_VirtualDom_pushPatch(patches, 8, rootIndex, {
			w: localPatches,
			x: inserts,
			y: endInserts
		});
	}
}



// CHANGES FROM KEYED DIFF


var _VirtualDom_POSTFIX = '_elmW6BL';


function _VirtualDom_insertNode(changes, localPatches, key, vnode, yIndex, inserts)
{
	var entry = changes[key];

	// never seen this key before
	if (!entry)
	{
		entry = {
			c: 0,
			z: vnode,
			r: yIndex,
			s: undefined
		};

		inserts.push({ r: yIndex, A: entry });
		changes[key] = entry;

		return;
	}

	// this key was removed earlier, a match!
	if (entry.c === 1)
	{
		inserts.push({ r: yIndex, A: entry });

		entry.c = 2;
		var subPatches = [];
		_VirtualDom_diffHelp(entry.z, vnode, subPatches, entry.r);
		entry.r = yIndex;
		entry.s.s = {
			w: subPatches,
			A: entry
		};

		return;
	}

	// this key has already been inserted or moved, a duplicate!
	_VirtualDom_insertNode(changes, localPatches, key + _VirtualDom_POSTFIX, vnode, yIndex, inserts);
}


function _VirtualDom_removeNode(changes, localPatches, key, vnode, index)
{
	var entry = changes[key];

	// never seen this key before
	if (!entry)
	{
		var patch = _VirtualDom_pushPatch(localPatches, 9, index, undefined);

		changes[key] = {
			c: 1,
			z: vnode,
			r: index,
			s: patch
		};

		return;
	}

	// this key was inserted earlier, a match!
	if (entry.c === 0)
	{
		entry.c = 2;
		var subPatches = [];
		_VirtualDom_diffHelp(vnode, entry.z, subPatches, index);

		_VirtualDom_pushPatch(localPatches, 9, index, {
			w: subPatches,
			A: entry
		});

		return;
	}

	// this key has already been removed or moved, a duplicate!
	_VirtualDom_removeNode(changes, localPatches, key + _VirtualDom_POSTFIX, vnode, index);
}



// ADD DOM NODES
//
// Each DOM node has an "index" assigned in order of traversal. It is important
// to minimize our crawl over the actual DOM, so these indexes (along with the
// descendantsCount of virtual nodes) let us skip touching entire subtrees of
// the DOM if we know there are no patches there.


function _VirtualDom_addDomNodes(domNode, vNode, patches, eventNode)
{
	_VirtualDom_addDomNodesHelp(domNode, vNode, patches, 0, 0, vNode.b, eventNode);
}


// assumes `patches` is non-empty and indexes increase monotonically.
function _VirtualDom_addDomNodesHelp(domNode, vNode, patches, i, low, high, eventNode)
{
	var patch = patches[i];
	var index = patch.r;

	while (index === low)
	{
		var patchType = patch.$;

		if (patchType === 1)
		{
			_VirtualDom_addDomNodes(domNode, vNode.k, patch.s, eventNode);
		}
		else if (patchType === 8)
		{
			patch.t = domNode;
			patch.u = eventNode;

			var subPatches = patch.s.w;
			if (subPatches.length > 0)
			{
				_VirtualDom_addDomNodesHelp(domNode, vNode, subPatches, 0, low, high, eventNode);
			}
		}
		else if (patchType === 9)
		{
			patch.t = domNode;
			patch.u = eventNode;

			var data = patch.s;
			if (data)
			{
				data.A.s = domNode;
				var subPatches = data.w;
				if (subPatches.length > 0)
				{
					_VirtualDom_addDomNodesHelp(domNode, vNode, subPatches, 0, low, high, eventNode);
				}
			}
		}
		else
		{
			patch.t = domNode;
			patch.u = eventNode;
		}

		i++;

		if (!(patch = patches[i]) || (index = patch.r) > high)
		{
			return i;
		}
	}

	var tag = vNode.$;

	if (tag === 4)
	{
		var subNode = vNode.k;

		while (subNode.$ === 4)
		{
			subNode = subNode.k;
		}

		return _VirtualDom_addDomNodesHelp(domNode, subNode, patches, i, low + 1, high, domNode.elm_event_node_ref);
	}

	// tag must be 1 or 2 at this point

	var vKids = vNode.e;
	var childNodes = domNode.childNodes;
	for (var j = 0; j < vKids.length; j++)
	{
		low++;
		var vKid = tag === 1 ? vKids[j] : vKids[j].b;
		var nextLow = low + (vKid.b || 0);
		if (low <= index && index <= nextLow)
		{
			i = _VirtualDom_addDomNodesHelp(childNodes[j], vKid, patches, i, low, nextLow, eventNode);
			if (!(patch = patches[i]) || (index = patch.r) > high)
			{
				return i;
			}
		}
		low = nextLow;
	}
	return i;
}



// APPLY PATCHES


function _VirtualDom_applyPatches(rootDomNode, oldVirtualNode, patches, eventNode)
{
	if (patches.length === 0)
	{
		return rootDomNode;
	}

	_VirtualDom_addDomNodes(rootDomNode, oldVirtualNode, patches, eventNode);
	return _VirtualDom_applyPatchesHelp(rootDomNode, patches);
}

function _VirtualDom_applyPatchesHelp(rootDomNode, patches)
{
	for (var i = 0; i < patches.length; i++)
	{
		var patch = patches[i];
		var localDomNode = patch.t
		var newNode = _VirtualDom_applyPatch(localDomNode, patch);
		if (localDomNode === rootDomNode)
		{
			rootDomNode = newNode;
		}
	}
	return rootDomNode;
}

function _VirtualDom_applyPatch(domNode, patch)
{
	switch (patch.$)
	{
		case 0:
			return _VirtualDom_applyPatchRedraw(domNode, patch.s, patch.u);

		case 4:
			_VirtualDom_applyFacts(domNode, patch.u, patch.s);
			return domNode;

		case 3:
			domNode.replaceData(0, domNode.length, patch.s);
			return domNode;

		case 1:
			return _VirtualDom_applyPatchesHelp(domNode, patch.s);

		case 2:
			if (domNode.elm_event_node_ref)
			{
				domNode.elm_event_node_ref.j = patch.s;
			}
			else
			{
				domNode.elm_event_node_ref = { j: patch.s, p: patch.u };
			}
			return domNode;

		case 6:
			var data = patch.s;
			for (var i = 0; i < data.i; i++)
			{
				domNode.removeChild(domNode.childNodes[data.v]);
			}
			return domNode;

		case 7:
			var data = patch.s;
			var kids = data.e;
			var i = data.v;
			var theEnd = domNode.childNodes[i];
			for (; i < kids.length; i++)
			{
				domNode.insertBefore(_VirtualDom_render(kids[i], patch.u), theEnd);
			}
			return domNode;

		case 9:
			var data = patch.s;
			if (!data)
			{
				domNode.parentNode.removeChild(domNode);
				return domNode;
			}
			var entry = data.A;
			if (typeof entry.r !== 'undefined')
			{
				domNode.parentNode.removeChild(domNode);
			}
			entry.s = _VirtualDom_applyPatchesHelp(domNode, data.w);
			return domNode;

		case 8:
			return _VirtualDom_applyPatchReorder(domNode, patch);

		case 5:
			return patch.s(domNode);

		default:
			_Debug_crash(10); // 'Ran into an unknown patch!'
	}
}


function _VirtualDom_applyPatchRedraw(domNode, vNode, eventNode)
{
	var parentNode = domNode.parentNode;
	var newNode = _VirtualDom_render(vNode, eventNode);

	if (!newNode.elm_event_node_ref)
	{
		newNode.elm_event_node_ref = domNode.elm_event_node_ref;
	}

	if (parentNode && newNode !== domNode)
	{
		parentNode.replaceChild(newNode, domNode);
	}
	return newNode;
}


function _VirtualDom_applyPatchReorder(domNode, patch)
{
	var data = patch.s;

	// remove end inserts
	var frag = _VirtualDom_applyPatchReorderEndInsertsHelp(data.y, patch);

	// removals
	domNode = _VirtualDom_applyPatchesHelp(domNode, data.w);

	// inserts
	var inserts = data.x;
	for (var i = 0; i < inserts.length; i++)
	{
		var insert = inserts[i];
		var entry = insert.A;
		var node = entry.c === 2
			? entry.s
			: _VirtualDom_render(entry.z, patch.u);
		domNode.insertBefore(node, domNode.childNodes[insert.r]);
	}

	// add end inserts
	if (frag)
	{
		_VirtualDom_appendChild(domNode, frag);
	}

	return domNode;
}


function _VirtualDom_applyPatchReorderEndInsertsHelp(endInserts, patch)
{
	if (!endInserts)
	{
		return;
	}

	var frag = _VirtualDom_doc.createDocumentFragment();
	for (var i = 0; i < endInserts.length; i++)
	{
		var insert = endInserts[i];
		var entry = insert.A;
		_VirtualDom_appendChild(frag, entry.c === 2
			? entry.s
			: _VirtualDom_render(entry.z, patch.u)
		);
	}
	return frag;
}


function _VirtualDom_virtualize(node)
{
	// TEXT NODES

	if (node.nodeType === 3)
	{
		return _VirtualDom_text(node.textContent);
	}


	// WEIRD NODES

	if (node.nodeType !== 1)
	{
		return _VirtualDom_text('');
	}


	// ELEMENT NODES

	var attrList = _List_Nil;
	var attrs = node.attributes;
	for (var i = attrs.length; i--; )
	{
		var attr = attrs[i];
		var name = attr.name;
		var value = attr.value;
		attrList = _List_Cons( A2(_VirtualDom_attribute, name, value), attrList );
	}

	var tag = node.tagName.toLowerCase();
	var kidList = _List_Nil;
	var kids = node.childNodes;

	for (var i = kids.length; i--; )
	{
		kidList = _List_Cons(_VirtualDom_virtualize(kids[i]), kidList);
	}
	return A3(_VirtualDom_node, tag, attrList, kidList);
}

function _VirtualDom_dekey(keyedNode)
{
	var keyedKids = keyedNode.e;
	var len = keyedKids.length;
	var kids = new Array(len);
	for (var i = 0; i < len; i++)
	{
		kids[i] = keyedKids[i].b;
	}

	return {
		$: 1,
		c: keyedNode.c,
		d: keyedNode.d,
		e: kids,
		f: keyedNode.f,
		b: keyedNode.b
	};
}




// ELEMENT


var _Debugger_element;

var _Browser_element = _Debugger_element || F4(function(impl, flagDecoder, debugMetadata, args)
{
	return _Platform_initialize(
		flagDecoder,
		args,
		impl.a0,
		impl.bc,
		impl.bb,
		function(sendToApp, initialModel) {
			var view = impl.be;
			/**/
			var domNode = args['node'];
			//*/
			/**_UNUSED/
			var domNode = args && args['node'] ? args['node'] : _Debug_crash(0);
			//*/
			var currNode = _VirtualDom_virtualize(domNode);

			return _Browser_makeAnimator(initialModel, function(model)
			{
				var nextNode = view(model);
				var patches = _VirtualDom_diff(currNode, nextNode);
				domNode = _VirtualDom_applyPatches(domNode, currNode, patches, sendToApp);
				currNode = nextNode;
			});
		}
	);
});



// DOCUMENT


var _Debugger_document;

var _Browser_document = _Debugger_document || F4(function(impl, flagDecoder, debugMetadata, args)
{
	return _Platform_initialize(
		flagDecoder,
		args,
		impl.a0,
		impl.bc,
		impl.bb,
		function(sendToApp, initialModel) {
			var divertHrefToApp = impl.E && impl.E(sendToApp)
			var view = impl.be;
			var title = _VirtualDom_doc.title;
			var bodyNode = _VirtualDom_doc.body;
			var currNode = _VirtualDom_virtualize(bodyNode);
			return _Browser_makeAnimator(initialModel, function(model)
			{
				_VirtualDom_divertHrefToApp = divertHrefToApp;
				var doc = view(model);
				var nextNode = _VirtualDom_node('body')(_List_Nil)(doc.aQ);
				var patches = _VirtualDom_diff(currNode, nextNode);
				bodyNode = _VirtualDom_applyPatches(bodyNode, currNode, patches, sendToApp);
				currNode = nextNode;
				_VirtualDom_divertHrefToApp = 0;
				(title !== doc.K) && (_VirtualDom_doc.title = title = doc.K);
			});
		}
	);
});



// ANIMATION


var _Browser_cancelAnimationFrame =
	typeof cancelAnimationFrame !== 'undefined'
		? cancelAnimationFrame
		: function(id) { clearTimeout(id); };

var _Browser_requestAnimationFrame =
	typeof requestAnimationFrame !== 'undefined'
		? requestAnimationFrame
		: function(callback) { return setTimeout(callback, 1000 / 60); };


function _Browser_makeAnimator(model, draw)
{
	draw(model);

	var state = 0;

	function updateIfNeeded()
	{
		state = state === 1
			? 0
			: ( _Browser_requestAnimationFrame(updateIfNeeded), draw(model), 1 );
	}

	return function(nextModel, isSync)
	{
		model = nextModel;

		isSync
			? ( draw(model),
				state === 2 && (state = 1)
				)
			: ( state === 0 && _Browser_requestAnimationFrame(updateIfNeeded),
				state = 2
				);
	};
}



// APPLICATION


function _Browser_application(impl)
{
	var onUrlChange = impl.a2;
	var onUrlRequest = impl.a3;
	var key = function() { key.a(onUrlChange(_Browser_getUrl())); };

	return _Browser_document({
		E: function(sendToApp)
		{
			key.a = sendToApp;
			_Browser_window.addEventListener('popstate', key);
			_Browser_window.navigator.userAgent.indexOf('Trident') < 0 || _Browser_window.addEventListener('hashchange', key);

			return F2(function(domNode, event)
			{
				if (!event.ctrlKey && !event.metaKey && !event.shiftKey && event.button < 1 && !domNode.target && !domNode.hasAttribute('download'))
				{
					event.preventDefault();
					var href = domNode.href;
					var curr = _Browser_getUrl();
					var next = elm$url$Url$fromString(href).a;
					sendToApp(onUrlRequest(
						(next
							&& curr.a6 === next.a6
							&& curr.ak === next.ak
							&& curr.at.a === next.at.a
						)
							? elm$browser$Browser$Internal(next)
							: elm$browser$Browser$External(href)
					));
				}
			});
		},
		a0: function(flags)
		{
			return A3(impl.a0, flags, _Browser_getUrl(), key);
		},
		be: impl.be,
		bc: impl.bc,
		bb: impl.bb
	});
}

function _Browser_getUrl()
{
	return elm$url$Url$fromString(_VirtualDom_doc.location.href).a || _Debug_crash(1);
}

var _Browser_go = F2(function(key, n)
{
	return A2(elm$core$Task$perform, elm$core$Basics$never, _Scheduler_binding(function() {
		n && history.go(n);
		key();
	}));
});

var _Browser_pushUrl = F2(function(key, url)
{
	return A2(elm$core$Task$perform, elm$core$Basics$never, _Scheduler_binding(function() {
		history.pushState({}, '', url);
		key();
	}));
});

var _Browser_replaceUrl = F2(function(key, url)
{
	return A2(elm$core$Task$perform, elm$core$Basics$never, _Scheduler_binding(function() {
		history.replaceState({}, '', url);
		key();
	}));
});



// GLOBAL EVENTS


var _Browser_fakeNode = { addEventListener: function() {}, removeEventListener: function() {} };
var _Browser_doc = typeof document !== 'undefined' ? document : _Browser_fakeNode;
var _Browser_window = typeof window !== 'undefined' ? window : _Browser_fakeNode;

var _Browser_on = F3(function(node, eventName, sendToSelf)
{
	return _Scheduler_spawn(_Scheduler_binding(function(callback)
	{
		function handler(event)	{ _Scheduler_rawSpawn(sendToSelf(event)); }
		node.addEventListener(eventName, handler, _VirtualDom_passiveSupported && { passive: true });
		return function() { node.removeEventListener(eventName, handler); };
	}));
});

var _Browser_decodeEvent = F2(function(decoder, event)
{
	var result = _Json_runHelp(decoder, event);
	return elm$core$Result$isOk(result) ? elm$core$Maybe$Just(result.a) : elm$core$Maybe$Nothing;
});



// PAGE VISIBILITY


function _Browser_visibilityInfo()
{
	return (typeof _VirtualDom_doc.hidden !== 'undefined')
		? { a_: 'hidden', aR: 'visibilitychange' }
		:
	(typeof _VirtualDom_doc.mozHidden !== 'undefined')
		? { a_: 'mozHidden', aR: 'mozvisibilitychange' }
		:
	(typeof _VirtualDom_doc.msHidden !== 'undefined')
		? { a_: 'msHidden', aR: 'msvisibilitychange' }
		:
	(typeof _VirtualDom_doc.webkitHidden !== 'undefined')
		? { a_: 'webkitHidden', aR: 'webkitvisibilitychange' }
		: { a_: 'hidden', aR: 'visibilitychange' };
}



// ANIMATION FRAMES


function _Browser_rAF()
{
	return _Scheduler_binding(function(callback)
	{
		var id = _Browser_requestAnimationFrame(function() {
			callback(_Scheduler_succeed(Date.now()));
		});

		return function() {
			_Browser_cancelAnimationFrame(id);
		};
	});
}


function _Browser_now()
{
	return _Scheduler_binding(function(callback)
	{
		callback(_Scheduler_succeed(Date.now()));
	});
}



// DOM STUFF


function _Browser_withNode(id, doStuff)
{
	return _Scheduler_binding(function(callback)
	{
		_Browser_requestAnimationFrame(function() {
			var node = document.getElementById(id);
			callback(node
				? _Scheduler_succeed(doStuff(node))
				: _Scheduler_fail(elm$browser$Browser$Dom$NotFound(id))
			);
		});
	});
}


function _Browser_withWindow(doStuff)
{
	return _Scheduler_binding(function(callback)
	{
		_Browser_requestAnimationFrame(function() {
			callback(_Scheduler_succeed(doStuff()));
		});
	});
}


// FOCUS and BLUR


var _Browser_call = F2(function(functionName, id)
{
	return _Browser_withNode(id, function(node) {
		node[functionName]();
		return _Utils_Tuple0;
	});
});



// WINDOW VIEWPORT


function _Browser_getViewport()
{
	return {
		aC: _Browser_getScene(),
		aJ: {
			_: _Browser_window.pageXOffset,
			aa: _Browser_window.pageYOffset,
			X: _Browser_doc.documentElement.clientWidth,
			M: _Browser_doc.documentElement.clientHeight
		}
	};
}

function _Browser_getScene()
{
	var body = _Browser_doc.body;
	var elem = _Browser_doc.documentElement;
	return {
		X: Math.max(body.scrollWidth, body.offsetWidth, elem.scrollWidth, elem.offsetWidth, elem.clientWidth),
		M: Math.max(body.scrollHeight, body.offsetHeight, elem.scrollHeight, elem.offsetHeight, elem.clientHeight)
	};
}

var _Browser_setViewport = F2(function(x, y)
{
	return _Browser_withWindow(function()
	{
		_Browser_window.scroll(x, y);
		return _Utils_Tuple0;
	});
});



// ELEMENT VIEWPORT


function _Browser_getViewportOf(id)
{
	return _Browser_withNode(id, function(node)
	{
		return {
			aC: {
				X: node.scrollWidth,
				M: node.scrollHeight
			},
			aJ: {
				_: node.scrollLeft,
				aa: node.scrollTop,
				X: node.clientWidth,
				M: node.clientHeight
			}
		};
	});
}


var _Browser_setViewportOf = F3(function(id, x, y)
{
	return _Browser_withNode(id, function(node)
	{
		node.scrollLeft = x;
		node.scrollTop = y;
		return _Utils_Tuple0;
	});
});



// ELEMENT


function _Browser_getElement(id)
{
	return _Browser_withNode(id, function(node)
	{
		var rect = node.getBoundingClientRect();
		var x = _Browser_window.pageXOffset;
		var y = _Browser_window.pageYOffset;
		return {
			aC: _Browser_getScene(),
			aJ: {
				_: x,
				aa: y,
				X: _Browser_doc.documentElement.clientWidth,
				M: _Browser_doc.documentElement.clientHeight
			},
			aV: {
				_: x + rect.left,
				aa: y + rect.top,
				X: rect.width,
				M: rect.height
			}
		};
	});
}



// LOAD and RELOAD


function _Browser_reload(skipCache)
{
	return A2(elm$core$Task$perform, elm$core$Basics$never, _Scheduler_binding(function(callback)
	{
		_VirtualDom_doc.location.reload(skipCache);
	}));
}

function _Browser_load(url)
{
	return A2(elm$core$Task$perform, elm$core$Basics$never, _Scheduler_binding(function(callback)
	{
		try
		{
			_Browser_window.location = url;
		}
		catch(err)
		{
			// Only Firefox can throw a NS_ERROR_MALFORMED_URI exception here.
			// Other browsers reload the page, so let's be consistent about that.
			_VirtualDom_doc.location.reload(false);
		}
	}));
}
var author$project$Main$ScpMsg = function (a) {
	return {$: 2, a: a};
};
var author$project$Main$TextSize = function (a) {
	return {$: 1, a: a};
};
var elm$core$Basics$identity = function (x) {
	return x;
};
var author$project$Main$requestTextSize = _Platform_outgoingPort('requestTextSize', elm$core$Basics$identity);
var author$project$Main$sendSocketCommand = _Platform_outgoingPort('sendSocketCommand', elm$core$Basics$identity);
var elm$core$List$foldl = F3(
	function (func, acc, list) {
		foldl:
		while (true) {
			if (!list.b) {
				return acc;
			} else {
				var x = list.a;
				var xs = list.b;
				var $temp$func = func,
					$temp$acc = A2(func, x, acc),
					$temp$list = xs;
				func = $temp$func;
				acc = $temp$acc;
				list = $temp$list;
				continue foldl;
			}
		}
	});
var elm$core$Array$branchFactor = 32;
var elm$core$Array$Array_elm_builtin = F4(
	function (a, b, c, d) {
		return {$: 0, a: a, b: b, c: c, d: d};
	});
var elm$core$Basics$EQ = 1;
var elm$core$Basics$GT = 2;
var elm$core$Basics$LT = 0;
var elm$core$Dict$foldr = F3(
	function (func, acc, t) {
		foldr:
		while (true) {
			if (t.$ === -2) {
				return acc;
			} else {
				var key = t.b;
				var value = t.c;
				var left = t.d;
				var right = t.e;
				var $temp$func = func,
					$temp$acc = A3(
					func,
					key,
					value,
					A3(elm$core$Dict$foldr, func, acc, right)),
					$temp$t = left;
				func = $temp$func;
				acc = $temp$acc;
				t = $temp$t;
				continue foldr;
			}
		}
	});
var elm$core$List$cons = _List_cons;
var elm$core$Dict$toList = function (dict) {
	return A3(
		elm$core$Dict$foldr,
		F3(
			function (key, value, list) {
				return A2(
					elm$core$List$cons,
					_Utils_Tuple2(key, value),
					list);
			}),
		_List_Nil,
		dict);
};
var elm$core$Dict$keys = function (dict) {
	return A3(
		elm$core$Dict$foldr,
		F3(
			function (key, value, keyList) {
				return A2(elm$core$List$cons, key, keyList);
			}),
		_List_Nil,
		dict);
};
var elm$core$Set$toList = function (_n0) {
	var dict = _n0;
	return elm$core$Dict$keys(dict);
};
var elm$core$Elm$JsArray$foldr = _JsArray_foldr;
var elm$core$Array$foldr = F3(
	function (func, baseCase, _n0) {
		var tree = _n0.c;
		var tail = _n0.d;
		var helper = F2(
			function (node, acc) {
				if (!node.$) {
					var subTree = node.a;
					return A3(elm$core$Elm$JsArray$foldr, helper, acc, subTree);
				} else {
					var values = node.a;
					return A3(elm$core$Elm$JsArray$foldr, func, acc, values);
				}
			});
		return A3(
			elm$core$Elm$JsArray$foldr,
			helper,
			A3(elm$core$Elm$JsArray$foldr, func, baseCase, tail),
			tree);
	});
var elm$core$Array$toList = function (array) {
	return A3(elm$core$Array$foldr, elm$core$List$cons, _List_Nil, array);
};
var elm$core$Basics$ceiling = _Basics_ceiling;
var elm$core$Basics$fdiv = _Basics_fdiv;
var elm$core$Basics$logBase = F2(
	function (base, number) {
		return _Basics_log(number) / _Basics_log(base);
	});
var elm$core$Basics$toFloat = _Basics_toFloat;
var elm$core$Array$shiftStep = elm$core$Basics$ceiling(
	A2(elm$core$Basics$logBase, 2, elm$core$Array$branchFactor));
var elm$core$Elm$JsArray$empty = _JsArray_empty;
var elm$core$Array$empty = A4(elm$core$Array$Array_elm_builtin, 0, elm$core$Array$shiftStep, elm$core$Elm$JsArray$empty, elm$core$Elm$JsArray$empty);
var elm$core$Array$Leaf = function (a) {
	return {$: 1, a: a};
};
var elm$core$Array$SubTree = function (a) {
	return {$: 0, a: a};
};
var elm$core$Elm$JsArray$initializeFromList = _JsArray_initializeFromList;
var elm$core$List$reverse = function (list) {
	return A3(elm$core$List$foldl, elm$core$List$cons, _List_Nil, list);
};
var elm$core$Array$compressNodes = F2(
	function (nodes, acc) {
		compressNodes:
		while (true) {
			var _n0 = A2(elm$core$Elm$JsArray$initializeFromList, elm$core$Array$branchFactor, nodes);
			var node = _n0.a;
			var remainingNodes = _n0.b;
			var newAcc = A2(
				elm$core$List$cons,
				elm$core$Array$SubTree(node),
				acc);
			if (!remainingNodes.b) {
				return elm$core$List$reverse(newAcc);
			} else {
				var $temp$nodes = remainingNodes,
					$temp$acc = newAcc;
				nodes = $temp$nodes;
				acc = $temp$acc;
				continue compressNodes;
			}
		}
	});
var elm$core$Basics$apR = F2(
	function (x, f) {
		return f(x);
	});
var elm$core$Basics$eq = _Utils_equal;
var elm$core$Tuple$first = function (_n0) {
	var x = _n0.a;
	return x;
};
var elm$core$Array$treeFromBuilder = F2(
	function (nodeList, nodeListSize) {
		treeFromBuilder:
		while (true) {
			var newNodeSize = elm$core$Basics$ceiling(nodeListSize / elm$core$Array$branchFactor);
			if (newNodeSize === 1) {
				return A2(elm$core$Elm$JsArray$initializeFromList, elm$core$Array$branchFactor, nodeList).a;
			} else {
				var $temp$nodeList = A2(elm$core$Array$compressNodes, nodeList, _List_Nil),
					$temp$nodeListSize = newNodeSize;
				nodeList = $temp$nodeList;
				nodeListSize = $temp$nodeListSize;
				continue treeFromBuilder;
			}
		}
	});
var elm$core$Basics$add = _Basics_add;
var elm$core$Basics$apL = F2(
	function (f, x) {
		return f(x);
	});
var elm$core$Basics$floor = _Basics_floor;
var elm$core$Basics$gt = _Utils_gt;
var elm$core$Basics$max = F2(
	function (x, y) {
		return (_Utils_cmp(x, y) > 0) ? x : y;
	});
var elm$core$Basics$mul = _Basics_mul;
var elm$core$Basics$sub = _Basics_sub;
var elm$core$Elm$JsArray$length = _JsArray_length;
var elm$core$Array$builderToArray = F2(
	function (reverseNodeList, builder) {
		if (!builder.a) {
			return A4(
				elm$core$Array$Array_elm_builtin,
				elm$core$Elm$JsArray$length(builder.b),
				elm$core$Array$shiftStep,
				elm$core$Elm$JsArray$empty,
				builder.b);
		} else {
			var treeLen = builder.a * elm$core$Array$branchFactor;
			var depth = elm$core$Basics$floor(
				A2(elm$core$Basics$logBase, elm$core$Array$branchFactor, treeLen - 1));
			var correctNodeList = reverseNodeList ? elm$core$List$reverse(builder.c) : builder.c;
			var tree = A2(elm$core$Array$treeFromBuilder, correctNodeList, builder.a);
			return A4(
				elm$core$Array$Array_elm_builtin,
				elm$core$Elm$JsArray$length(builder.b) + treeLen,
				A2(elm$core$Basics$max, 5, depth * elm$core$Array$shiftStep),
				tree,
				builder.b);
		}
	});
var elm$core$Basics$False = 1;
var elm$core$Basics$idiv = _Basics_idiv;
var elm$core$Basics$lt = _Utils_lt;
var elm$core$Elm$JsArray$initialize = _JsArray_initialize;
var elm$core$Array$initializeHelp = F5(
	function (fn, fromIndex, len, nodeList, tail) {
		initializeHelp:
		while (true) {
			if (fromIndex < 0) {
				return A2(
					elm$core$Array$builderToArray,
					false,
					{c: nodeList, a: (len / elm$core$Array$branchFactor) | 0, b: tail});
			} else {
				var leaf = elm$core$Array$Leaf(
					A3(elm$core$Elm$JsArray$initialize, elm$core$Array$branchFactor, fromIndex, fn));
				var $temp$fn = fn,
					$temp$fromIndex = fromIndex - elm$core$Array$branchFactor,
					$temp$len = len,
					$temp$nodeList = A2(elm$core$List$cons, leaf, nodeList),
					$temp$tail = tail;
				fn = $temp$fn;
				fromIndex = $temp$fromIndex;
				len = $temp$len;
				nodeList = $temp$nodeList;
				tail = $temp$tail;
				continue initializeHelp;
			}
		}
	});
var elm$core$Basics$le = _Utils_le;
var elm$core$Basics$remainderBy = _Basics_remainderBy;
var elm$core$Array$initialize = F2(
	function (len, fn) {
		if (len <= 0) {
			return elm$core$Array$empty;
		} else {
			var tailLen = len % elm$core$Array$branchFactor;
			var tail = A3(elm$core$Elm$JsArray$initialize, tailLen, len - tailLen, fn);
			var initialFromIndex = (len - tailLen) - elm$core$Array$branchFactor;
			return A5(elm$core$Array$initializeHelp, fn, initialFromIndex, len, _List_Nil, tail);
		}
	});
var elm$core$Maybe$Just = function (a) {
	return {$: 0, a: a};
};
var elm$core$Maybe$Nothing = {$: 1};
var elm$core$Result$Err = function (a) {
	return {$: 1, a: a};
};
var elm$core$Result$Ok = function (a) {
	return {$: 0, a: a};
};
var elm$core$Basics$True = 0;
var elm$core$Result$isOk = function (result) {
	if (!result.$) {
		return true;
	} else {
		return false;
	}
};
var elm$json$Json$Decode$Failure = F2(
	function (a, b) {
		return {$: 3, a: a, b: b};
	});
var elm$json$Json$Decode$Field = F2(
	function (a, b) {
		return {$: 0, a: a, b: b};
	});
var elm$json$Json$Decode$Index = F2(
	function (a, b) {
		return {$: 1, a: a, b: b};
	});
var elm$json$Json$Decode$OneOf = function (a) {
	return {$: 2, a: a};
};
var elm$core$Basics$and = _Basics_and;
var elm$core$Basics$append = _Utils_append;
var elm$core$Basics$or = _Basics_or;
var elm$core$Char$toCode = _Char_toCode;
var elm$core$Char$isLower = function (_char) {
	var code = elm$core$Char$toCode(_char);
	return (97 <= code) && (code <= 122);
};
var elm$core$Char$isUpper = function (_char) {
	var code = elm$core$Char$toCode(_char);
	return (code <= 90) && (65 <= code);
};
var elm$core$Char$isAlpha = function (_char) {
	return elm$core$Char$isLower(_char) || elm$core$Char$isUpper(_char);
};
var elm$core$Char$isDigit = function (_char) {
	var code = elm$core$Char$toCode(_char);
	return (code <= 57) && (48 <= code);
};
var elm$core$Char$isAlphaNum = function (_char) {
	return elm$core$Char$isLower(_char) || (elm$core$Char$isUpper(_char) || elm$core$Char$isDigit(_char));
};
var elm$core$List$length = function (xs) {
	return A3(
		elm$core$List$foldl,
		F2(
			function (_n0, i) {
				return i + 1;
			}),
		0,
		xs);
};
var elm$core$List$map2 = _List_map2;
var elm$core$List$rangeHelp = F3(
	function (lo, hi, list) {
		rangeHelp:
		while (true) {
			if (_Utils_cmp(lo, hi) < 1) {
				var $temp$lo = lo,
					$temp$hi = hi - 1,
					$temp$list = A2(elm$core$List$cons, hi, list);
				lo = $temp$lo;
				hi = $temp$hi;
				list = $temp$list;
				continue rangeHelp;
			} else {
				return list;
			}
		}
	});
var elm$core$List$range = F2(
	function (lo, hi) {
		return A3(elm$core$List$rangeHelp, lo, hi, _List_Nil);
	});
var elm$core$List$indexedMap = F2(
	function (f, xs) {
		return A3(
			elm$core$List$map2,
			f,
			A2(
				elm$core$List$range,
				0,
				elm$core$List$length(xs) - 1),
			xs);
	});
var elm$core$String$all = _String_all;
var elm$core$String$fromInt = _String_fromNumber;
var elm$core$String$join = F2(
	function (sep, chunks) {
		return A2(
			_String_join,
			sep,
			_List_toArray(chunks));
	});
var elm$core$String$uncons = _String_uncons;
var elm$core$String$split = F2(
	function (sep, string) {
		return _List_fromArray(
			A2(_String_split, sep, string));
	});
var elm$json$Json$Decode$indent = function (str) {
	return A2(
		elm$core$String$join,
		'\n    ',
		A2(elm$core$String$split, '\n', str));
};
var elm$json$Json$Encode$encode = _Json_encode;
var elm$json$Json$Decode$errorOneOf = F2(
	function (i, error) {
		return '\n\n(' + (elm$core$String$fromInt(i + 1) + (') ' + elm$json$Json$Decode$indent(
			elm$json$Json$Decode$errorToString(error))));
	});
var elm$json$Json$Decode$errorToString = function (error) {
	return A2(elm$json$Json$Decode$errorToStringHelp, error, _List_Nil);
};
var elm$json$Json$Decode$errorToStringHelp = F2(
	function (error, context) {
		errorToStringHelp:
		while (true) {
			switch (error.$) {
				case 0:
					var f = error.a;
					var err = error.b;
					var isSimple = function () {
						var _n1 = elm$core$String$uncons(f);
						if (_n1.$ === 1) {
							return false;
						} else {
							var _n2 = _n1.a;
							var _char = _n2.a;
							var rest = _n2.b;
							return elm$core$Char$isAlpha(_char) && A2(elm$core$String$all, elm$core$Char$isAlphaNum, rest);
						}
					}();
					var fieldName = isSimple ? ('.' + f) : ('[\'' + (f + '\']'));
					var $temp$error = err,
						$temp$context = A2(elm$core$List$cons, fieldName, context);
					error = $temp$error;
					context = $temp$context;
					continue errorToStringHelp;
				case 1:
					var i = error.a;
					var err = error.b;
					var indexName = '[' + (elm$core$String$fromInt(i) + ']');
					var $temp$error = err,
						$temp$context = A2(elm$core$List$cons, indexName, context);
					error = $temp$error;
					context = $temp$context;
					continue errorToStringHelp;
				case 2:
					var errors = error.a;
					if (!errors.b) {
						return 'Ran into a Json.Decode.oneOf with no possibilities' + function () {
							if (!context.b) {
								return '!';
							} else {
								return ' at json' + A2(
									elm$core$String$join,
									'',
									elm$core$List$reverse(context));
							}
						}();
					} else {
						if (!errors.b.b) {
							var err = errors.a;
							var $temp$error = err,
								$temp$context = context;
							error = $temp$error;
							context = $temp$context;
							continue errorToStringHelp;
						} else {
							var starter = function () {
								if (!context.b) {
									return 'Json.Decode.oneOf';
								} else {
									return 'The Json.Decode.oneOf at json' + A2(
										elm$core$String$join,
										'',
										elm$core$List$reverse(context));
								}
							}();
							var introduction = starter + (' failed in the following ' + (elm$core$String$fromInt(
								elm$core$List$length(errors)) + ' ways:'));
							return A2(
								elm$core$String$join,
								'\n\n',
								A2(
									elm$core$List$cons,
									introduction,
									A2(elm$core$List$indexedMap, elm$json$Json$Decode$errorOneOf, errors)));
						}
					}
				default:
					var msg = error.a;
					var json = error.b;
					var introduction = function () {
						if (!context.b) {
							return 'Problem with the given value:\n\n';
						} else {
							return 'Problem with the value at json' + (A2(
								elm$core$String$join,
								'',
								elm$core$List$reverse(context)) + ':\n\n    ');
						}
					}();
					return introduction + (elm$json$Json$Decode$indent(
						A2(elm$json$Json$Encode$encode, 4, json)) + ('\n\n' + msg));
			}
		}
	});
var elm$json$Json$Encode$object = function (pairs) {
	return _Json_wrap(
		A3(
			elm$core$List$foldl,
			F2(
				function (_n0, obj) {
					var k = _n0.a;
					var v = _n0.b;
					return A3(_Json_addField, k, v, obj);
				}),
			_Json_emptyObject(0),
			pairs));
};
var elm$json$Json$Encode$string = _Json_wrap;
var author$project$WebSocket$encodeCmd = function (wsc) {
	switch (wsc.$) {
		case 0:
			var msg = wsc.a;
			return elm$json$Json$Encode$object(
				_List_fromArray(
					[
						_Utils_Tuple2(
						'cmd',
						elm$json$Json$Encode$string('connect')),
						_Utils_Tuple2(
						'name',
						elm$json$Json$Encode$string(msg.ap)),
						_Utils_Tuple2(
						'address',
						elm$json$Json$Encode$string(msg.aO)),
						_Utils_Tuple2(
						'protocol',
						elm$json$Json$Encode$string(msg.a6))
					]));
		case 1:
			var msg = wsc.a;
			return elm$json$Json$Encode$object(
				_List_fromArray(
					[
						_Utils_Tuple2(
						'cmd',
						elm$json$Json$Encode$string('send')),
						_Utils_Tuple2(
						'name',
						elm$json$Json$Encode$string(msg.ap)),
						_Utils_Tuple2(
						'content',
						elm$json$Json$Encode$string(msg.aT))
					]));
		default:
			var msg = wsc.a;
			return elm$json$Json$Encode$object(
				_List_fromArray(
					[
						_Utils_Tuple2(
						'cmd',
						elm$json$Json$Encode$string('close')),
						_Utils_Tuple2(
						'name',
						elm$json$Json$Encode$string(msg.ap))
					]));
	}
};
var author$project$WebSocket$send = F2(
	function (tocmd, wsc) {
		return tocmd(
			author$project$WebSocket$encodeCmd(wsc));
	});
var author$project$Main$wssend = author$project$WebSocket$send(author$project$Main$sendSocketCommand);
var elm$json$Json$Encode$int = _Json_wrap;
var elm$json$Json$Encode$list = F2(
	function (func, entries) {
		return _Json_wrap(
			A3(
				elm$core$List$foldl,
				_Json_addEntry(func),
				_Json_emptyArray(0),
				entries));
	});
var author$project$SvgThings$encodeControlId = function (cid) {
	return A2(elm$json$Json$Encode$list, elm$json$Json$Encode$int, cid);
};
var author$project$SvgTextSize$encodeTextSizeRequest = function (tsr) {
	return elm$json$Json$Encode$object(
		_List_fromArray(
			[
				_Utils_Tuple2(
				'string',
				elm$json$Json$Encode$string(tsr.ba)),
				_Utils_Tuple2(
				'font',
				elm$json$Json$Encode$string(tsr.aZ)),
				_Utils_Tuple2(
				'controlId',
				author$project$SvgThings$encodeControlId(tsr.ae))
			]));
};
var author$project$WebSocket$Send = function (a) {
	return {$: 1, a: a};
};
var elm$core$List$foldrHelper = F4(
	function (fn, acc, ctr, ls) {
		if (!ls.b) {
			return acc;
		} else {
			var a = ls.a;
			var r1 = ls.b;
			if (!r1.b) {
				return A2(fn, a, acc);
			} else {
				var b = r1.a;
				var r2 = r1.b;
				if (!r2.b) {
					return A2(
						fn,
						a,
						A2(fn, b, acc));
				} else {
					var c = r2.a;
					var r3 = r2.b;
					if (!r3.b) {
						return A2(
							fn,
							a,
							A2(
								fn,
								b,
								A2(fn, c, acc)));
					} else {
						var d = r3.a;
						var r4 = r3.b;
						var res = (ctr > 500) ? A3(
							elm$core$List$foldl,
							fn,
							acc,
							elm$core$List$reverse(r4)) : A4(elm$core$List$foldrHelper, fn, acc, ctr + 1, r4);
						return A2(
							fn,
							a,
							A2(
								fn,
								b,
								A2(
									fn,
									c,
									A2(fn, d, res))));
					}
				}
			}
		}
	});
var elm$core$List$foldr = F3(
	function (fn, acc, ls) {
		return A4(elm$core$List$foldrHelper, fn, acc, 0, ls);
	});
var elm$core$List$map = F2(
	function (f, xs) {
		return A3(
			elm$core$List$foldr,
			F2(
				function (x, acc) {
					return A2(
						elm$core$List$cons,
						f(x),
						acc);
				}),
			_List_Nil,
			xs);
	});
var elm$core$Platform$Cmd$batch = _Platform_batch;
var elm$core$Platform$Cmd$none = elm$core$Platform$Cmd$batch(_List_Nil);
var author$project$Main$commandToCmd = function (scmd) {
	switch (scmd.$) {
		case 0:
			var dta = scmd.a;
			return author$project$Main$wssend(
				author$project$WebSocket$Send(
					{aT: dta, ap: 'touchpage'}));
		case 1:
			var rtw = scmd.a;
			return author$project$Main$requestTextSize(
				author$project$SvgTextSize$encodeTextSizeRequest(rtw));
		case 2:
			return elm$core$Platform$Cmd$none;
		default:
			var cmds = scmd.a;
			return elm$core$Platform$Cmd$batch(
				A2(elm$core$List$map, author$project$Main$commandToCmd, cmds));
	}
};
var author$project$SvgControl$CsLabel = function (a) {
	return {$: 3, a: a};
};
var author$project$SvgControlPage$Spec = F8(
	function (title, rootControl, state, controlsColor, labelsColor, textColor, pressedColor, unpressedColor) {
		return {af: controlsColor, an: labelsColor, au: pressedColor, aB: rootControl, aD: state, aG: textColor, K: title, aI: unpressedColor};
	});
var author$project$SvgCommand$Batch = function (a) {
	return {$: 3, a: a};
};
var author$project$SvgButton$Model = F9(
	function (name, label, stringWidth, cid, rect, srect, pressed, textSvg, touchonly) {
		return {ad: cid, q: label, ap: name, n: pressed, ay: rect, y: srect, T: stringWidth, J: textSvg, aH: touchonly};
	});
var author$project$SvgCommand$None = {$: 2};
var author$project$SvgCommand$RequestTextWidth = function (a) {
	return {$: 1, a: a};
};
var author$project$SvgCommand$TextSizeRequest = F3(
	function (string, font, controlId) {
		return {ae: controlId, aZ: font, ba: string};
	});
var author$project$SvgTextSize$controlFontFamily = 'sans-serif';
var author$project$SvgTextSize$sizingFont = '20px ' + author$project$SvgTextSize$controlFontFamily;
var author$project$SvgTextSize$resizeCommand = function (model) {
	var _n0 = model.T;
	if (_n0.$ === 1) {
		return (model.q === '') ? author$project$SvgCommand$None : author$project$SvgCommand$RequestTextWidth(
			A3(author$project$SvgCommand$TextSizeRequest, model.q, author$project$SvgTextSize$sizingFont, model.ad));
	} else {
		return author$project$SvgCommand$None;
	}
};
var author$project$SvgThings$SRect = F4(
	function (x, y, w, h) {
		return {L: h, W: w, _: x, aa: y};
	});
var elm$core$Maybe$withDefault = F2(
	function (_default, maybe) {
		if (!maybe.$) {
			var value = maybe.a;
			return value;
		} else {
			return _default;
		}
	});
var author$project$SvgButton$init = F3(
	function (rect, cid, spec) {
		var model = A9(
			author$project$SvgButton$Model,
			spec.ap,
			A2(elm$core$Maybe$withDefault, '', spec.q),
			elm$core$Maybe$Nothing,
			cid,
			rect,
			A4(
				author$project$SvgThings$SRect,
				elm$core$String$fromInt(rect._),
				elm$core$String$fromInt(rect.aa),
				elm$core$String$fromInt(rect.W),
				elm$core$String$fromInt(rect.L)),
			false,
			_List_Nil,
			false);
		return _Utils_Tuple2(
			model,
			author$project$SvgTextSize$resizeCommand(model));
	});
var author$project$SvgControl$CmButton = function (a) {
	return {$: 0, a: a};
};
var author$project$SvgControl$CmLabel = function (a) {
	return {$: 3, a: a};
};
var author$project$SvgControl$CmSizer = function (a) {
	return {$: 4, a: a};
};
var author$project$SvgControl$CmSlider = function (a) {
	return {$: 1, a: a};
};
var author$project$SvgControl$CmXY = function (a) {
	return {$: 2, a: a};
};
var author$project$SvgControl$SzModel = F5(
	function (cid, rect, controls, orientation, proportions) {
		return {ad: cid, d: controls, C: orientation, D: proportions, ay: rect};
	});
var author$project$SvgControl$border = 1;
var author$project$SvgThings$Rect = F4(
	function (x, y, w, h) {
		return {L: h, W: w, _: x, aa: y};
	});
var author$project$SvgThings$mekhr = F3(
	function (br, w, i) {
		return A4(author$project$SvgThings$Rect, br._ + (w * i), br.aa, w, br.L);
	});
var elm$core$Basics$round = _Basics_round;
var author$project$SvgThings$hrects = F2(
	function (rct, count) {
		var w = elm$core$Basics$round(rct.W / count);
		var idxs = A2(elm$core$List$range, 0, count - 1);
		return A2(
			elm$core$List$map,
			A2(author$project$SvgThings$mekhr, rct, w),
			idxs);
	});
var author$project$SvgThings$mekhrp = F2(
	function (prect, _n0) {
		var x = _n0.a;
		var w = _n0.b;
		return A4(author$project$SvgThings$Rect, x, prect.aa, w, prect.L);
	});
var elm$core$List$append = F2(
	function (xs, ys) {
		if (!ys.b) {
			return xs;
		} else {
			return A3(elm$core$List$foldr, elm$core$List$cons, ys, xs);
		}
	});
var elm$core$List$repeatHelp = F3(
	function (result, n, value) {
		repeatHelp:
		while (true) {
			if (n <= 0) {
				return result;
			} else {
				var $temp$result = A2(elm$core$List$cons, value, result),
					$temp$n = n - 1,
					$temp$value = value;
				result = $temp$result;
				n = $temp$n;
				value = $temp$value;
				continue repeatHelp;
			}
		}
	});
var elm$core$List$repeat = F2(
	function (n, value) {
		return A3(elm$core$List$repeatHelp, _List_Nil, n, value);
	});
var elm$core$List$sum = function (numbers) {
	return A3(elm$core$List$foldl, elm$core$Basics$add, 0, numbers);
};
var elm$core$List$takeReverse = F3(
	function (n, list, kept) {
		takeReverse:
		while (true) {
			if (n <= 0) {
				return kept;
			} else {
				if (!list.b) {
					return kept;
				} else {
					var x = list.a;
					var xs = list.b;
					var $temp$n = n - 1,
						$temp$list = xs,
						$temp$kept = A2(elm$core$List$cons, x, kept);
					n = $temp$n;
					list = $temp$list;
					kept = $temp$kept;
					continue takeReverse;
				}
			}
		}
	});
var elm$core$List$takeTailRec = F2(
	function (n, list) {
		return elm$core$List$reverse(
			A3(elm$core$List$takeReverse, n, list, _List_Nil));
	});
var elm$core$List$takeFast = F3(
	function (ctr, n, list) {
		if (n <= 0) {
			return _List_Nil;
		} else {
			var _n0 = _Utils_Tuple2(n, list);
			_n0$1:
			while (true) {
				_n0$5:
				while (true) {
					if (!_n0.b.b) {
						return list;
					} else {
						if (_n0.b.b.b) {
							switch (_n0.a) {
								case 1:
									break _n0$1;
								case 2:
									var _n2 = _n0.b;
									var x = _n2.a;
									var _n3 = _n2.b;
									var y = _n3.a;
									return _List_fromArray(
										[x, y]);
								case 3:
									if (_n0.b.b.b.b) {
										var _n4 = _n0.b;
										var x = _n4.a;
										var _n5 = _n4.b;
										var y = _n5.a;
										var _n6 = _n5.b;
										var z = _n6.a;
										return _List_fromArray(
											[x, y, z]);
									} else {
										break _n0$5;
									}
								default:
									if (_n0.b.b.b.b && _n0.b.b.b.b.b) {
										var _n7 = _n0.b;
										var x = _n7.a;
										var _n8 = _n7.b;
										var y = _n8.a;
										var _n9 = _n8.b;
										var z = _n9.a;
										var _n10 = _n9.b;
										var w = _n10.a;
										var tl = _n10.b;
										return (ctr > 1000) ? A2(
											elm$core$List$cons,
											x,
											A2(
												elm$core$List$cons,
												y,
												A2(
													elm$core$List$cons,
													z,
													A2(
														elm$core$List$cons,
														w,
														A2(elm$core$List$takeTailRec, n - 4, tl))))) : A2(
											elm$core$List$cons,
											x,
											A2(
												elm$core$List$cons,
												y,
												A2(
													elm$core$List$cons,
													z,
													A2(
														elm$core$List$cons,
														w,
														A3(elm$core$List$takeFast, ctr + 1, n - 4, tl)))));
									} else {
										break _n0$5;
									}
							}
						} else {
							if (_n0.a === 1) {
								break _n0$1;
							} else {
								break _n0$5;
							}
						}
					}
				}
				return list;
			}
			var _n1 = _n0.b;
			var x = _n1.a;
			return _List_fromArray(
				[x]);
		}
	});
var elm$core$List$take = F2(
	function (n, list) {
		return A3(elm$core$List$takeFast, 0, n, list);
	});
var author$project$SvgThings$processProps = F2(
	function (controlcount, lst) {
		var l = elm$core$List$length(lst);
		var r = (_Utils_cmp(controlcount, l) > 0) ? (controlcount - l) : 0;
		var nwlst = A2(
			elm$core$List$append,
			A2(elm$core$List$take, controlcount, lst),
			A2(elm$core$List$repeat, r, 0.0));
		var s = elm$core$List$sum(nwlst);
		return A2(
			elm$core$List$map,
			function (x) {
				return x / s;
			},
			nwlst);
	});
var elm$core$List$head = function (list) {
	if (list.b) {
		var x = list.a;
		var xs = list.b;
		return elm$core$Maybe$Just(x);
	} else {
		return elm$core$Maybe$Nothing;
	}
};
var elm$core$List$tail = function (list) {
	if (list.b) {
		var x = list.a;
		var xs = list.b;
		return elm$core$Maybe$Just(xs);
	} else {
		return elm$core$Maybe$Nothing;
	}
};
var author$project$SvgThings$somme = F2(
	function (f, lst) {
		var _n0 = elm$core$List$head(lst);
		if (_n0.$ === 1) {
			return lst;
		} else {
			var hf = _n0.a;
			var tl = elm$core$List$tail(lst);
			var s = f + hf;
			if (tl.$ === 1) {
				return _List_fromArray(
					[s]);
			} else {
				var t = tl.a;
				return A2(
					elm$core$List$cons,
					f,
					A2(author$project$SvgThings$somme, s, t));
			}
		}
	});
var elm$core$Tuple$pair = F2(
	function (a, b) {
		return _Utils_Tuple2(a, b);
	});
var author$project$SvgThings$hrectsp = F3(
	function (rct, count, props) {
		var pprops = A2(author$project$SvgThings$processProps, count, props);
		var fw = rct.W;
		var widths = A2(
			elm$core$List$map,
			function (p) {
				return elm$core$Basics$round(p * fw);
			},
			pprops);
		var xes = A2(author$project$SvgThings$somme, rct._, widths);
		return A2(
			elm$core$List$map,
			author$project$SvgThings$mekhrp(rct),
			A3(elm$core$List$map2, elm$core$Tuple$pair, xes, widths));
	});
var author$project$SvgThings$mekvr = F3(
	function (br, h, i) {
		return A4(author$project$SvgThings$Rect, br._, br.aa + (h * i), br.W, h);
	});
var author$project$SvgThings$vrects = F2(
	function (rct, count) {
		var idxs = A2(elm$core$List$range, 0, count - 1);
		var h = elm$core$Basics$round(rct.L / count);
		return A2(
			elm$core$List$map,
			A2(author$project$SvgThings$mekvr, rct, h),
			idxs);
	});
var author$project$SvgThings$mekvrp = F2(
	function (prect, _n0) {
		var y = _n0.a;
		var h = _n0.b;
		return A4(author$project$SvgThings$Rect, prect._, y, prect.W, h);
	});
var author$project$SvgThings$vrectsp = F3(
	function (rct, count, props) {
		var pprops = A2(author$project$SvgThings$processProps, count, props);
		var fh = rct.L;
		var heights = A2(
			elm$core$List$map,
			function (p) {
				return elm$core$Basics$round(p * fh);
			},
			pprops);
		var yes = A2(author$project$SvgThings$somme, rct.aa, heights);
		return A2(
			elm$core$List$map,
			author$project$SvgThings$mekvrp(rct),
			A3(elm$core$List$map2, elm$core$Tuple$pair, yes, heights));
	});
var author$project$SvgControl$mkRlist = F4(
	function (orientation, rect, count, mbproportions) {
		if (orientation === 1) {
			if (mbproportions.$ === 1) {
				return A2(author$project$SvgThings$hrects, rect, count);
			} else {
				var p = mbproportions.a;
				return A3(author$project$SvgThings$hrectsp, rect, count, p);
			}
		} else {
			if (mbproportions.$ === 1) {
				return A2(author$project$SvgThings$vrects, rect, count);
			} else {
				var p = mbproportions.a;
				return A3(author$project$SvgThings$vrectsp, rect, count, p);
			}
		}
	});
var author$project$SvgControl$zip = elm$core$List$map2(elm$core$Tuple$pair);
var author$project$SvgLabel$Model = F7(
	function (name, label, stringWidth, cid, rect, srect, textSvg) {
		return {ad: cid, q: label, ap: name, ay: rect, y: srect, T: stringWidth, J: textSvg};
	});
var author$project$SvgLabel$init = F3(
	function (rect, cid, spec) {
		var model = A7(
			author$project$SvgLabel$Model,
			spec.ap,
			spec.q,
			elm$core$Maybe$Nothing,
			cid,
			rect,
			A4(
				author$project$SvgThings$SRect,
				elm$core$String$fromInt(rect._),
				elm$core$String$fromInt(rect.aa),
				elm$core$String$fromInt(rect.W),
				elm$core$String$fromInt(rect.L)),
			_List_Nil);
		return _Utils_Tuple2(
			model,
			author$project$SvgTextSize$resizeCommand(model));
	});
var author$project$SvgSlider$Model = function (name) {
	return function (label) {
		return function (stringWidth) {
			return function (cid) {
				return function (rect) {
					return function (srect) {
						return function (orientation) {
							return function (pressed) {
								return function (location) {
									return function (textSvg) {
										return function (touchonly) {
											return {ad: cid, q: label, N: location, ap: name, C: orientation, n: pressed, ay: rect, y: srect, T: stringWidth, J: textSvg, aH: touchonly};
										};
									};
								};
							};
						};
					};
				};
			};
		};
	};
};
var author$project$SvgSlider$init = F3(
	function (rect, cid, spec) {
		var model = author$project$SvgSlider$Model(spec.ap)(
			A2(elm$core$Maybe$withDefault, '', spec.q))(elm$core$Maybe$Nothing)(cid)(rect)(
			A4(
				author$project$SvgThings$SRect,
				elm$core$String$fromInt(rect._),
				elm$core$String$fromInt(rect.aa),
				elm$core$String$fromInt(rect.W),
				elm$core$String$fromInt(rect.L)))(spec.C)(false)(0.5)(_List_Nil)(false);
		return _Utils_Tuple2(
			model,
			author$project$SvgTextSize$resizeCommand(model));
	});
var author$project$SvgThings$shrinkRect = F2(
	function (border, rect) {
		return A4(author$project$SvgThings$Rect, rect._ + border, rect.aa + border, (rect.W - border) - border, (rect.L - border) - border);
	});
var author$project$SvgXY$Model = function (name) {
	return function (label) {
		return function (stringWidth) {
			return function (textSvg) {
				return function (cid) {
					return function (rect) {
						return function (srect) {
							return function (pressed) {
								return function (location) {
									return function (touchonly) {
										return {ad: cid, q: label, N: location, ap: name, n: pressed, ay: rect, y: srect, T: stringWidth, J: textSvg, aH: touchonly};
									};
								};
							};
						};
					};
				};
			};
		};
	};
};
var author$project$SvgXY$init = F3(
	function (rect, cid, spec) {
		var model = author$project$SvgXY$Model(spec.ap)(
			A2(elm$core$Maybe$withDefault, '', spec.q))(elm$core$Maybe$Nothing)(_List_Nil)(cid)(rect)(
			A4(
				author$project$SvgThings$SRect,
				elm$core$String$fromInt(rect._),
				elm$core$String$fromInt(rect.aa),
				elm$core$String$fromInt(rect.W),
				elm$core$String$fromInt(rect.L)))(false)(
			_Utils_Tuple2(0.5, 0.5))(false);
		return _Utils_Tuple2(
			model,
			author$project$SvgTextSize$resizeCommand(model));
	});
var elm$core$Dict$RBEmpty_elm_builtin = {$: -2};
var elm$core$Dict$empty = elm$core$Dict$RBEmpty_elm_builtin;
var elm$core$Dict$Black = 1;
var elm$core$Dict$RBNode_elm_builtin = F5(
	function (a, b, c, d, e) {
		return {$: -1, a: a, b: b, c: c, d: d, e: e};
	});
var elm$core$Basics$compare = _Utils_compare;
var elm$core$Dict$Red = 0;
var elm$core$Dict$balance = F5(
	function (color, key, value, left, right) {
		if ((right.$ === -1) && (!right.a)) {
			var _n1 = right.a;
			var rK = right.b;
			var rV = right.c;
			var rLeft = right.d;
			var rRight = right.e;
			if ((left.$ === -1) && (!left.a)) {
				var _n3 = left.a;
				var lK = left.b;
				var lV = left.c;
				var lLeft = left.d;
				var lRight = left.e;
				return A5(
					elm$core$Dict$RBNode_elm_builtin,
					0,
					key,
					value,
					A5(elm$core$Dict$RBNode_elm_builtin, 1, lK, lV, lLeft, lRight),
					A5(elm$core$Dict$RBNode_elm_builtin, 1, rK, rV, rLeft, rRight));
			} else {
				return A5(
					elm$core$Dict$RBNode_elm_builtin,
					color,
					rK,
					rV,
					A5(elm$core$Dict$RBNode_elm_builtin, 0, key, value, left, rLeft),
					rRight);
			}
		} else {
			if ((((left.$ === -1) && (!left.a)) && (left.d.$ === -1)) && (!left.d.a)) {
				var _n5 = left.a;
				var lK = left.b;
				var lV = left.c;
				var _n6 = left.d;
				var _n7 = _n6.a;
				var llK = _n6.b;
				var llV = _n6.c;
				var llLeft = _n6.d;
				var llRight = _n6.e;
				var lRight = left.e;
				return A5(
					elm$core$Dict$RBNode_elm_builtin,
					0,
					lK,
					lV,
					A5(elm$core$Dict$RBNode_elm_builtin, 1, llK, llV, llLeft, llRight),
					A5(elm$core$Dict$RBNode_elm_builtin, 1, key, value, lRight, right));
			} else {
				return A5(elm$core$Dict$RBNode_elm_builtin, color, key, value, left, right);
			}
		}
	});
var elm$core$Dict$insertHelp = F3(
	function (key, value, dict) {
		if (dict.$ === -2) {
			return A5(elm$core$Dict$RBNode_elm_builtin, 0, key, value, elm$core$Dict$RBEmpty_elm_builtin, elm$core$Dict$RBEmpty_elm_builtin);
		} else {
			var nColor = dict.a;
			var nKey = dict.b;
			var nValue = dict.c;
			var nLeft = dict.d;
			var nRight = dict.e;
			var _n1 = A2(elm$core$Basics$compare, key, nKey);
			switch (_n1) {
				case 0:
					return A5(
						elm$core$Dict$balance,
						nColor,
						nKey,
						nValue,
						A3(elm$core$Dict$insertHelp, key, value, nLeft),
						nRight);
				case 1:
					return A5(elm$core$Dict$RBNode_elm_builtin, nColor, nKey, value, nLeft, nRight);
				default:
					return A5(
						elm$core$Dict$balance,
						nColor,
						nKey,
						nValue,
						nLeft,
						A3(elm$core$Dict$insertHelp, key, value, nRight));
			}
		}
	});
var elm$core$Dict$insert = F3(
	function (key, value, dict) {
		var _n0 = A3(elm$core$Dict$insertHelp, key, value, dict);
		if ((_n0.$ === -1) && (!_n0.a)) {
			var _n1 = _n0.a;
			var k = _n0.b;
			var v = _n0.c;
			var l = _n0.d;
			var r = _n0.e;
			return A5(elm$core$Dict$RBNode_elm_builtin, 1, k, v, l, r);
		} else {
			var x = _n0;
			return x;
		}
	});
var elm$core$Dict$fromList = function (assocs) {
	return A3(
		elm$core$List$foldl,
		F2(
			function (_n0, dict) {
				var key = _n0.a;
				var value = _n0.b;
				return A3(elm$core$Dict$insert, key, value, dict);
			}),
		elm$core$Dict$empty,
		assocs);
};
var elm$core$List$map3 = _List_map3;
var elm$core$Tuple$second = function (_n0) {
	var y = _n0.b;
	return y;
};
var author$project$SvgControl$init = F3(
	function (rect, cid, spec) {
		var aptg = F2(
			function (f, _n2) {
				var m = _n2.a;
				var c = _n2.b;
				return _Utils_Tuple2(
					f(m),
					c);
			});
		switch (spec.$) {
			case 0:
				var s = spec.a;
				return A2(
					aptg,
					author$project$SvgControl$CmButton,
					A3(
						author$project$SvgButton$init,
						A2(author$project$SvgThings$shrinkRect, author$project$SvgControl$border, rect),
						cid,
						s));
			case 1:
				var s = spec.a;
				return A2(
					aptg,
					author$project$SvgControl$CmSlider,
					A3(
						author$project$SvgSlider$init,
						A2(author$project$SvgThings$shrinkRect, author$project$SvgControl$border, rect),
						cid,
						s));
			case 2:
				var s = spec.a;
				return A2(
					aptg,
					author$project$SvgControl$CmXY,
					A3(
						author$project$SvgXY$init,
						A2(author$project$SvgThings$shrinkRect, author$project$SvgControl$border, rect),
						cid,
						s));
			case 3:
				var s = spec.a;
				return A2(
					aptg,
					author$project$SvgControl$CmLabel,
					A3(
						author$project$SvgLabel$init,
						A2(author$project$SvgThings$shrinkRect, author$project$SvgControl$border, rect),
						cid,
						s));
			default:
				var s = spec.a;
				return A2(
					aptg,
					author$project$SvgControl$CmSizer,
					A3(author$project$SvgControl$szinit, rect, cid, s));
		}
	});
var author$project$SvgControl$szinit = F3(
	function (rect, cid, szspec) {
		var rlist = A4(
			author$project$SvgControl$mkRlist,
			szspec.C,
			rect,
			elm$core$List$length(szspec.d),
			szspec.D);
		var idxs = A2(
			elm$core$List$range,
			0,
			elm$core$List$length(szspec.d));
		var blist = A2(
			elm$core$List$map,
			function (_n0) {
				var spec = _n0.a;
				var rect_ = _n0.b;
				var idx = _n0.c;
				return A3(
					author$project$SvgControl$init,
					rect_,
					_Utils_ap(
						cid,
						_List_fromArray(
							[idx])),
					spec);
			},
			A4(
				elm$core$List$map3,
				F3(
					function (a, b, c) {
						return _Utils_Tuple3(a, b, c);
					}),
				szspec.d,
				rlist,
				idxs));
		var cmds = A2(elm$core$List$map, elm$core$Tuple$second, blist);
		var mods = A2(elm$core$List$map, elm$core$Tuple$first, blist);
		var controlz = A2(author$project$SvgControl$zip, idxs, mods);
		var model = A5(
			author$project$SvgControl$SzModel,
			cid,
			rect,
			elm$core$Dict$fromList(controlz),
			szspec.C,
			szspec.D);
		return _Utils_Tuple2(
			model,
			author$project$SvgCommand$Batch(cmds));
	});
var author$project$SvgButton$Press = 0;
var author$project$SvgButton$Unpress = 1;
var author$project$SvgButton$UpdateMessage = F3(
	function (controlId, updateType, label) {
		return {ae: controlId, q: label, V: updateType};
	});
var author$project$SvgButton$encodeUpdateType = function (ut) {
	if (!ut) {
		return elm$json$Json$Encode$string('Press');
	} else {
		return elm$json$Json$Encode$string('Unpress');
	}
};
var author$project$SvgButton$encodeUpdateMessage = function (um) {
	var outlist1 = _List_fromArray(
		[
			_Utils_Tuple2(
			'controlType',
			elm$json$Json$Encode$string('button')),
			_Utils_Tuple2(
			'controlId',
			author$project$SvgThings$encodeControlId(um.ae))
		]);
	var outlist2 = function () {
		var _n1 = um.V;
		if (!_n1.$) {
			var ut = _n1.a;
			return A2(
				elm$core$List$append,
				outlist1,
				_List_fromArray(
					[
						_Utils_Tuple2(
						'state',
						author$project$SvgButton$encodeUpdateType(ut))
					]));
		} else {
			return outlist1;
		}
	}();
	var outlist3 = function () {
		var _n0 = um.q;
		if (!_n0.$) {
			var txt = _n0.a;
			return A2(
				elm$core$List$append,
				outlist2,
				_List_fromArray(
					[
						_Utils_Tuple2(
						'label',
						elm$json$Json$Encode$string(txt))
					]));
		} else {
			return outlist2;
		}
	}();
	return elm$json$Json$Encode$object(outlist3);
};
var author$project$SvgCommand$Send = function (a) {
	return {$: 0, a: a};
};
var author$project$SvgButton$pressup = F2(
	function (model, ut) {
		var um = A2(
			elm$json$Json$Encode$encode,
			0,
			author$project$SvgButton$encodeUpdateMessage(
				A3(
					author$project$SvgButton$UpdateMessage,
					model.ad,
					elm$core$Maybe$Just(ut),
					elm$core$Maybe$Nothing)));
		return _Utils_Tuple2(
			_Utils_update(
				model,
				{n: !ut}),
			author$project$SvgCommand$Send(um));
	});
var author$project$SvgTouch$Touch = F3(
	function (x, y, id) {
		return {al: id, _: x, aa: y};
	});
var elm$json$Json$Decode$field = _Json_decodeField;
var elm$json$Json$Decode$float = _Json_decodeFloat;
var elm$json$Json$Decode$int = _Json_decodeInt;
var elm$json$Json$Decode$map3 = _Json_map3;
var author$project$SvgTouch$parseTouch = A4(
	elm$json$Json$Decode$map3,
	author$project$SvgTouch$Touch,
	A2(elm$json$Json$Decode$field, 'clientX', elm$json$Json$Decode$float),
	A2(elm$json$Json$Decode$field, 'clientY', elm$json$Json$Decode$float),
	A2(elm$json$Json$Decode$field, 'identifier', elm$json$Json$Decode$int));
var author$project$SvgTouch$stDebugLog = F2(
	function (a, b) {
		return b;
	});
var elm$json$Json$Decode$at = F2(
	function (fields, decoder) {
		return A3(elm$core$List$foldr, elm$json$Json$Decode$field, decoder, fields);
	});
var elm$json$Json$Decode$decodeValue = _Json_run;
var author$project$SvgTouch$extractFirstTouch = function (evt) {
	var _n0 = A2(
		elm$json$Json$Decode$decodeValue,
		A2(
			elm$json$Json$Decode$at,
			_List_fromArray(
				['touches', '0']),
			author$project$SvgTouch$parseTouch),
		evt);
	if (!_n0.$) {
		var touch = _n0.a;
		return elm$core$Maybe$Just(touch);
	} else {
		var e = _n0.a;
		return A2(
			author$project$SvgTouch$stDebugLog,
			elm$json$Json$Decode$errorToString(e),
			elm$core$Maybe$Nothing);
	}
};
var author$project$SvgTouch$extractFirstTouchSE = function (msg) {
	switch (msg.$) {
		case 0:
			var v = msg.a;
			return author$project$SvgTouch$extractFirstTouch(v);
		case 1:
			var v = msg.a;
			return author$project$SvgTouch$extractFirstTouch(v);
		case 2:
			var v = msg.a;
			return elm$core$Maybe$Nothing;
		case 3:
			var v = msg.a;
			return elm$core$Maybe$Nothing;
		default:
			var v = msg.a;
			return elm$core$Maybe$Nothing;
	}
};
var author$project$SvgButton$update = F2(
	function (msg, model) {
		switch (msg.$) {
			case 0:
				return A2(author$project$SvgButton$pressup, model, 0);
			case 1:
				return model.n ? A2(author$project$SvgButton$pressup, model, 1) : _Utils_Tuple2(model, author$project$SvgCommand$None);
			case 2:
				return _Utils_Tuple2(model, author$project$SvgCommand$None);
			case 3:
				var s = msg.a;
				return _Utils_Tuple2(
					_Utils_update(
						model,
						{ap: s}),
					author$project$SvgCommand$None);
			case 5:
				var um = msg.a;
				var nm1 = _Utils_update(
					model,
					{
						n: function () {
							var _n2 = um.V;
							if (!_n2.$) {
								if (!_n2.a) {
									var _n3 = _n2.a;
									return true;
								} else {
									var _n4 = _n2.a;
									return false;
								}
							} else {
								return model.n;
							}
						}()
					});
				var nm2 = function () {
					var _n1 = um.q;
					if (!_n1.$) {
						var txt = _n1.a;
						return _Utils_update(
							nm1,
							{q: txt, T: elm$core$Maybe$Nothing, J: _List_Nil});
					} else {
						return nm1;
					}
				}();
				return _Utils_Tuple2(
					nm2,
					author$project$SvgTextSize$resizeCommand(nm2));
			default:
				var stm = msg.a;
				var _n5 = author$project$SvgTouch$extractFirstTouchSE(stm);
				if (_n5.$ === 1) {
					return model.n ? A2(author$project$SvgButton$pressup, model, 1) : _Utils_Tuple2(model, author$project$SvgCommand$None);
				} else {
					return (!model.n) ? A2(author$project$SvgButton$pressup, model, 0) : _Utils_Tuple2(model, author$project$SvgCommand$None);
				}
		}
	});
var author$project$SvgLabel$update = F2(
	function (msg, model) {
		if (!msg.$) {
			var um = msg.a;
			var newmodel = _Utils_update(
				model,
				{q: um.q, T: elm$core$Maybe$Nothing, J: _List_Nil});
			return _Utils_Tuple2(
				newmodel,
				author$project$SvgTextSize$resizeCommand(newmodel));
		} else {
			return _Utils_Tuple2(model, author$project$SvgCommand$None);
		}
	});
var author$project$SvgSlider$Press = 0;
var author$project$SvgSlider$Unpress = 1;
var author$project$SvgSlider$getX = A2(elm$json$Json$Decode$field, 'pageX', elm$json$Json$Decode$int);
var author$project$SvgSlider$getY = A2(elm$json$Json$Decode$field, 'pageY', elm$json$Json$Decode$int);
var author$project$SvgSlider$getLocation = F2(
	function (model, v) {
		var _n0 = model.C;
		if (_n0 === 1) {
			var _n1 = A2(elm$json$Json$Decode$decodeValue, author$project$SvgSlider$getX, v);
			if (!_n1.$) {
				var i = _n1.a;
				return elm$core$Result$Ok((i - model.ay._) / model.ay.W);
			} else {
				var e = _n1.a;
				return elm$core$Result$Err(
					elm$json$Json$Decode$errorToString(e));
			}
		} else {
			var _n2 = A2(elm$json$Json$Decode$decodeValue, author$project$SvgSlider$getY, v);
			if (!_n2.$) {
				var i = _n2.a;
				return elm$core$Result$Ok((i - model.ay.aa) / model.ay.L);
			} else {
				var e = _n2.a;
				return elm$core$Result$Err(
					elm$json$Json$Decode$errorToString(e));
			}
		}
	});
var author$project$SvgSlider$UpdateMessage = F4(
	function (controlId, updateType, location, label) {
		return {ae: controlId, q: label, N: location, V: updateType};
	});
var author$project$SvgSlider$encodeUpdateType = function (ut) {
	if (!ut) {
		return elm$json$Json$Encode$string('Press');
	} else {
		return elm$json$Json$Encode$string('Unpress');
	}
};
var elm$json$Json$Encode$float = _Json_wrap;
var author$project$SvgSlider$encodeUpdateMessage = function (um) {
	var outlist1 = _List_fromArray(
		[
			_Utils_Tuple2(
			'controlType',
			elm$json$Json$Encode$string('slider')),
			_Utils_Tuple2(
			'controlId',
			author$project$SvgThings$encodeControlId(um.ae))
		]);
	var outlist2 = function () {
		var _n2 = um.V;
		if (!_n2.$) {
			var ut = _n2.a;
			return A2(
				elm$core$List$append,
				outlist1,
				_List_fromArray(
					[
						_Utils_Tuple2(
						'state',
						author$project$SvgSlider$encodeUpdateType(ut))
					]));
		} else {
			return outlist1;
		}
	}();
	var outlist3 = function () {
		var _n1 = um.N;
		if (!_n1.$) {
			var loc = _n1.a;
			return A2(
				elm$core$List$append,
				outlist2,
				_List_fromArray(
					[
						_Utils_Tuple2(
						'location',
						elm$json$Json$Encode$float(loc))
					]));
		} else {
			return outlist2;
		}
	}();
	var outlist4 = function () {
		var _n0 = um.q;
		if (!_n0.$) {
			var txt = _n0.a;
			return A2(
				elm$core$List$append,
				outlist3,
				_List_fromArray(
					[
						_Utils_Tuple2(
						'label',
						elm$json$Json$Encode$string(txt))
					]));
		} else {
			return outlist3;
		}
	}();
	return elm$json$Json$Encode$object(outlist4);
};
var elm$core$Basics$neq = _Utils_notEqual;
var author$project$SvgSlider$updsend = F3(
	function (model, mbut, loc) {
		var prest = !_Utils_eq(
			mbut,
			elm$core$Maybe$Just(1));
		var bLoc = (loc > 1.0) ? 1.0 : ((loc < 0.0) ? 0.0 : loc);
		if (_Utils_eq(model.N, bLoc) && _Utils_eq(model.n, prest)) {
			return _Utils_Tuple2(model, author$project$SvgCommand$None);
		} else {
			var um = A2(
				elm$json$Json$Encode$encode,
				0,
				author$project$SvgSlider$encodeUpdateMessage(
					A4(
						author$project$SvgSlider$UpdateMessage,
						model.ad,
						mbut,
						elm$core$Maybe$Just(bLoc),
						elm$core$Maybe$Nothing)));
			return _Utils_Tuple2(
				_Utils_update(
					model,
					{N: bLoc, n: prest}),
				author$project$SvgCommand$Send(um));
		}
	});
var elm$core$Basics$ge = _Utils_ge;
var author$project$SvgThings$containsXY = F3(
	function (rect, x, y) {
		return (_Utils_cmp(rect._, x) < 1) && ((_Utils_cmp(rect.W, x - rect._) > -1) && ((_Utils_cmp(rect.aa, y) < 1) && (_Utils_cmp(rect.L, y - rect.aa) > -1)));
	});
var author$project$SvgTouch$parseTouchCount = A2(
	elm$json$Json$Decode$at,
	_List_fromArray(
		['touches', 'length']),
	elm$json$Json$Decode$int);
var author$project$SvgTouch$extractTouches = function (evt) {
	var _n0 = A2(elm$json$Json$Decode$decodeValue, author$project$SvgTouch$parseTouchCount, evt);
	if (!_n0.$) {
		var touchcount = _n0.a;
		var touchresults = A2(
			elm$core$List$map,
			function (idx) {
				return A2(
					elm$json$Json$Decode$decodeValue,
					A2(
						elm$json$Json$Decode$at,
						_List_fromArray(
							[
								'touches',
								elm$core$String$fromInt(idx)
							]),
						author$project$SvgTouch$parseTouch),
					evt);
			},
			A2(elm$core$List$range, 0, touchcount - 1));
		var touches = A3(
			elm$core$List$foldr,
			F2(
				function (rst, tl) {
					if (!rst.$) {
						var touch = rst.a;
						return A2(elm$core$List$cons, touch, tl);
					} else {
						var e = rst.a;
						return A2(
							author$project$SvgTouch$stDebugLog,
							elm$json$Json$Decode$errorToString(e),
							tl);
					}
				}),
			_List_Nil,
			touchresults);
		return touches;
	} else {
		var str_msg = _n0.a;
		return A2(
			author$project$SvgTouch$stDebugLog,
			elm$json$Json$Decode$errorToString(str_msg),
			_List_Nil);
	}
};
var elm$core$Basics$truncate = _Basics_truncate;
var elm$core$List$filter = F2(
	function (isGood, list) {
		return A3(
			elm$core$List$foldr,
			F2(
				function (x, xs) {
					return isGood(x) ? A2(elm$core$List$cons, x, xs) : xs;
				}),
			_List_Nil,
			list);
	});
var author$project$SvgTouch$extractFirstTouchInRect = F2(
	function (evt, rect) {
		var touches = author$project$SvgTouch$extractTouches(evt);
		return elm$core$List$head(
			A2(
				elm$core$List$filter,
				function (touch) {
					return A3(author$project$SvgThings$containsXY, rect, touch._ | 0, touch.aa | 0);
				},
				touches));
	});
var author$project$SvgTouch$extractFirstRectTouchSE = F2(
	function (msg, rect) {
		switch (msg.$) {
			case 0:
				var v = msg.a;
				return A2(author$project$SvgTouch$extractFirstTouchInRect, v, rect);
			case 1:
				var v = msg.a;
				return A2(author$project$SvgTouch$extractFirstTouchInRect, v, rect);
			case 2:
				var v = msg.a;
				return elm$core$Maybe$Nothing;
			case 3:
				var v = msg.a;
				return elm$core$Maybe$Nothing;
			default:
				var v = msg.a;
				return elm$core$Maybe$Nothing;
		}
	});
var author$project$SvgSlider$update = F2(
	function (msg, model) {
		switch (msg.$) {
			case 0:
				var v = msg.a;
				var _n1 = A2(author$project$SvgSlider$getLocation, model, v);
				if (!_n1.$) {
					var l = _n1.a;
					return A3(
						author$project$SvgSlider$updsend,
						model,
						elm$core$Maybe$Just(0),
						l);
				} else {
					return _Utils_Tuple2(model, author$project$SvgCommand$None);
				}
			case 1:
				var v = msg.a;
				var _n2 = model.n;
				if (_n2) {
					return A3(
						author$project$SvgSlider$updsend,
						model,
						elm$core$Maybe$Just(1),
						model.N);
				} else {
					return _Utils_Tuple2(model, author$project$SvgCommand$None);
				}
			case 2:
				return _Utils_Tuple2(model, author$project$SvgCommand$None);
			case 3:
				var s = msg.a;
				return _Utils_Tuple2(
					_Utils_update(
						model,
						{ap: s}),
					author$project$SvgCommand$None);
			case 4:
				var v = msg.a;
				var _n3 = model.n;
				if (_n3) {
					var _n4 = A2(author$project$SvgSlider$getLocation, model, v);
					if (!_n4.$) {
						var l = _n4.a;
						return A3(author$project$SvgSlider$updsend, model, elm$core$Maybe$Nothing, l);
					} else {
						return _Utils_Tuple2(model, author$project$SvgCommand$None);
					}
				} else {
					return _Utils_Tuple2(model, author$project$SvgCommand$None);
				}
			case 6:
				var um = msg.a;
				var mod = _Utils_update(
					model,
					{
						N: function () {
							var _n6 = um.N;
							if (!_n6.$) {
								var loc = _n6.a;
								return loc;
							} else {
								return model.N;
							}
						}(),
						n: function () {
							var _n7 = um.V;
							if (!_n7.$) {
								if (!_n7.a) {
									var _n8 = _n7.a;
									return true;
								} else {
									var _n9 = _n7.a;
									return false;
								}
							} else {
								return model.n;
							}
						}()
					});
				var mod2 = function () {
					var _n5 = um.q;
					if (!_n5.$) {
						var txt = _n5.a;
						return _Utils_update(
							mod,
							{q: txt, T: elm$core$Maybe$Nothing, J: _List_Nil});
					} else {
						return mod;
					}
				}();
				return _Utils_Tuple2(
					mod2,
					author$project$SvgTextSize$resizeCommand(mod2));
			default:
				var stm = msg.a;
				var _n10 = A2(author$project$SvgTouch$extractFirstRectTouchSE, stm, model.ay);
				if (_n10.$ === 1) {
					return model.n ? A3(
						author$project$SvgSlider$updsend,
						model,
						elm$core$Maybe$Just(1),
						model.N) : _Utils_Tuple2(model, author$project$SvgCommand$None);
				} else {
					var touch = _n10.a;
					var _n11 = model.C;
					if (_n11 === 1) {
						var loc = (touch._ - model.ay._) / model.ay.W;
						return model.n ? A3(
							author$project$SvgSlider$updsend,
							model,
							elm$core$Maybe$Just(0),
							loc) : A3(author$project$SvgSlider$updsend, model, elm$core$Maybe$Nothing, loc);
					} else {
						var loc = (touch.aa - model.ay.aa) / model.ay.L;
						return model.n ? A3(
							author$project$SvgSlider$updsend,
							model,
							elm$core$Maybe$Just(0),
							loc) : A3(author$project$SvgSlider$updsend, model, elm$core$Maybe$Nothing, loc);
					}
				}
		}
	});
var author$project$SvgXY$Press = 0;
var author$project$SvgXY$Unpress = 1;
var author$project$SvgXY$getX = A2(elm$json$Json$Decode$field, 'pageX', elm$json$Json$Decode$int);
var author$project$SvgXY$getY = A2(elm$json$Json$Decode$field, 'pageY', elm$json$Json$Decode$int);
var elm$core$Result$andThen = F2(
	function (callback, result) {
		if (!result.$) {
			var value = result.a;
			return callback(value);
		} else {
			var msg = result.a;
			return elm$core$Result$Err(msg);
		}
	});
var elm$core$Result$map = F2(
	function (func, ra) {
		if (!ra.$) {
			var a = ra.a;
			return elm$core$Result$Ok(
				func(a));
		} else {
			var e = ra.a;
			return elm$core$Result$Err(e);
		}
	});
var author$project$SvgXY$getLocation = F2(
	function (model, v) {
		var yr = function () {
			var _n1 = A2(elm$json$Json$Decode$decodeValue, author$project$SvgXY$getY, v);
			if (!_n1.$) {
				var i = _n1.a;
				return elm$core$Result$Ok((i - model.ay.aa) / model.ay.L);
			} else {
				var e = _n1.a;
				return elm$core$Result$Err(
					elm$json$Json$Decode$errorToString(e));
			}
		}();
		var xr = function () {
			var _n0 = A2(elm$json$Json$Decode$decodeValue, author$project$SvgXY$getX, v);
			if (!_n0.$) {
				var i = _n0.a;
				return elm$core$Result$Ok((i - model.ay._) / model.ay.W);
			} else {
				var e = _n0.a;
				return elm$core$Result$Err(
					elm$json$Json$Decode$errorToString(e));
			}
		}();
		return A2(
			elm$core$Result$andThen,
			function (x) {
				return A2(
					elm$core$Result$map,
					function (y) {
						return _Utils_Tuple2(x, y);
					},
					yr);
			},
			xr);
	});
var author$project$SvgXY$UpdateMessage = F4(
	function (controlId, updateType, location, label) {
		return {ae: controlId, q: label, N: location, V: updateType};
	});
var author$project$SvgXY$encodeUpdateType = function (ut) {
	if (!ut) {
		return elm$json$Json$Encode$string('Press');
	} else {
		return elm$json$Json$Encode$string('Unpress');
	}
};
var author$project$SvgXY$encodeUpdateMessage = function (um) {
	var outlist1 = _List_fromArray(
		[
			_Utils_Tuple2(
			'controlType',
			elm$json$Json$Encode$string('xy')),
			_Utils_Tuple2(
			'controlId',
			author$project$SvgThings$encodeControlId(um.ae))
		]);
	var outlist2 = function () {
		var _n3 = um.V;
		if (!_n3.$) {
			var ut = _n3.a;
			return A2(
				elm$core$List$append,
				outlist1,
				_List_fromArray(
					[
						_Utils_Tuple2(
						'state',
						author$project$SvgXY$encodeUpdateType(ut))
					]));
		} else {
			return outlist1;
		}
	}();
	var outlist3 = function () {
		var _n1 = um.N;
		if (!_n1.$) {
			var _n2 = _n1.a;
			var locx = _n2.a;
			var locy = _n2.b;
			return A2(
				elm$core$List$append,
				outlist2,
				_List_fromArray(
					[
						_Utils_Tuple2(
						'location',
						A2(
							elm$json$Json$Encode$list,
							elm$json$Json$Encode$float,
							_List_fromArray(
								[locx, locy])))
					]));
		} else {
			return outlist2;
		}
	}();
	var outlist4 = function () {
		var _n0 = um.q;
		if (!_n0.$) {
			var txt = _n0.a;
			return A2(
				elm$core$List$append,
				outlist3,
				_List_fromArray(
					[
						_Utils_Tuple2(
						'label',
						elm$json$Json$Encode$string(txt))
					]));
		} else {
			return outlist3;
		}
	}();
	return elm$json$Json$Encode$object(outlist4);
};
var author$project$SvgXY$updsend = F3(
	function (model, mbut, _n0) {
		var x = _n0.a;
		var y = _n0.b;
		var prest = !_Utils_eq(
			mbut,
			elm$core$Maybe$Just(1));
		var lim = function (loc) {
			return (loc > 1.0) ? 1.0 : ((loc < 0.0) ? 0.0 : loc);
		};
		var limloc = _Utils_Tuple2(
			lim(x),
			lim(y));
		if (_Utils_eq(model.N, limloc) && _Utils_eq(model.n, prest)) {
			return _Utils_Tuple2(model, author$project$SvgCommand$None);
		} else {
			var um = A2(
				elm$json$Json$Encode$encode,
				0,
				author$project$SvgXY$encodeUpdateMessage(
					A4(
						author$project$SvgXY$UpdateMessage,
						model.ad,
						mbut,
						elm$core$Maybe$Just(limloc),
						elm$core$Maybe$Nothing)));
			return _Utils_Tuple2(
				_Utils_update(
					model,
					{N: limloc, n: prest}),
				author$project$SvgCommand$Send(um));
		}
	});
var author$project$SvgXY$update = F2(
	function (msg, model) {
		switch (msg.$) {
			case 0:
				var v = msg.a;
				var _n1 = A2(author$project$SvgXY$getLocation, model, v);
				if (!_n1.$) {
					var l = _n1.a;
					return A3(
						author$project$SvgXY$updsend,
						model,
						elm$core$Maybe$Just(0),
						l);
				} else {
					return _Utils_Tuple2(model, author$project$SvgCommand$None);
				}
			case 1:
				var v = msg.a;
				var _n2 = model.n;
				if (_n2) {
					return A3(
						author$project$SvgXY$updsend,
						model,
						elm$core$Maybe$Just(1),
						model.N);
				} else {
					return _Utils_Tuple2(model, author$project$SvgCommand$None);
				}
			case 2:
				return _Utils_Tuple2(model, author$project$SvgCommand$None);
			case 3:
				var s = msg.a;
				return _Utils_Tuple2(
					_Utils_update(
						model,
						{ap: s}),
					author$project$SvgCommand$None);
			case 4:
				var v = msg.a;
				var _n3 = model.n;
				if (_n3) {
					var _n4 = A2(author$project$SvgXY$getLocation, model, v);
					if (!_n4.$) {
						var l = _n4.a;
						return A3(author$project$SvgXY$updsend, model, elm$core$Maybe$Nothing, l);
					} else {
						return _Utils_Tuple2(model, author$project$SvgCommand$None);
					}
				} else {
					return _Utils_Tuple2(model, author$project$SvgCommand$None);
				}
			case 6:
				var um = msg.a;
				var mod = _Utils_update(
					model,
					{
						N: function () {
							var _n6 = um.N;
							if (!_n6.$) {
								var loc = _n6.a;
								return loc;
							} else {
								return model.N;
							}
						}(),
						n: function () {
							var _n7 = um.V;
							if (!_n7.$) {
								if (!_n7.a) {
									var _n8 = _n7.a;
									return true;
								} else {
									var _n9 = _n7.a;
									return false;
								}
							} else {
								return model.n;
							}
						}()
					});
				var mod2 = function () {
					var _n5 = um.q;
					if (!_n5.$) {
						var txt = _n5.a;
						return _Utils_update(
							mod,
							{q: txt, T: elm$core$Maybe$Nothing, J: _List_Nil});
					} else {
						return mod;
					}
				}();
				return _Utils_Tuple2(
					mod2,
					author$project$SvgTextSize$resizeCommand(mod2));
			default:
				var stm = msg.a;
				var _n10 = A2(author$project$SvgTouch$extractFirstRectTouchSE, stm, model.ay);
				if (_n10.$ === 1) {
					return model.n ? A3(
						author$project$SvgXY$updsend,
						model,
						elm$core$Maybe$Just(1),
						model.N) : _Utils_Tuple2(model, author$project$SvgCommand$None);
				} else {
					var touch = _n10.a;
					var locy = (touch.aa - model.ay.aa) / model.ay.L;
					var locx = (touch._ - model.ay._) / model.ay.W;
					var loc = _Utils_Tuple2(locx, locy);
					return model.n ? A3(
						author$project$SvgXY$updsend,
						model,
						elm$core$Maybe$Just(0),
						loc) : A3(author$project$SvgXY$updsend, model, elm$core$Maybe$Nothing, loc);
				}
		}
	});
var elm$core$Dict$get = F2(
	function (targetKey, dict) {
		get:
		while (true) {
			if (dict.$ === -2) {
				return elm$core$Maybe$Nothing;
			} else {
				var key = dict.b;
				var value = dict.c;
				var left = dict.d;
				var right = dict.e;
				var _n1 = A2(elm$core$Basics$compare, targetKey, key);
				switch (_n1) {
					case 0:
						var $temp$targetKey = targetKey,
							$temp$dict = left;
						targetKey = $temp$targetKey;
						dict = $temp$dict;
						continue get;
					case 1:
						return elm$core$Maybe$Just(value);
					default:
						var $temp$targetKey = targetKey,
							$temp$dict = right;
						targetKey = $temp$targetKey;
						dict = $temp$dict;
						continue get;
				}
			}
		}
	});
var author$project$SvgControl$szupdate = F2(
	function (msg, model) {
		var id = msg.a;
		var act = msg.b;
		var bb = A2(elm$core$Dict$get, id, model.d);
		if (!bb.$) {
			var bm = bb.a;
			var wha = A2(author$project$SvgControl$update, act, bm);
			var updcontrols = A3(elm$core$Dict$insert, id, wha.a, model.d);
			var newmod = _Utils_update(
				model,
				{d: updcontrols});
			return _Utils_Tuple2(newmod, wha.b);
		} else {
			return _Utils_Tuple2(model, author$project$SvgCommand$None);
		}
	});
var author$project$SvgControl$update = F2(
	function (msg, model) {
		var _n0 = _Utils_Tuple2(msg, model);
		_n0$5:
		while (true) {
			switch (_n0.a.$) {
				case 0:
					if (!_n0.b.$) {
						var ms = _n0.a.a;
						var m = _n0.b.a;
						var _n1 = A2(author$project$SvgButton$update, ms, m);
						var a = _n1.a;
						var b = _n1.b;
						return _Utils_Tuple2(
							author$project$SvgControl$CmButton(a),
							b);
					} else {
						break _n0$5;
					}
				case 1:
					if (_n0.b.$ === 1) {
						var ms = _n0.a.a;
						var m = _n0.b.a;
						var _n2 = A2(author$project$SvgSlider$update, ms, m);
						var a = _n2.a;
						var b = _n2.b;
						return _Utils_Tuple2(
							author$project$SvgControl$CmSlider(a),
							b);
					} else {
						break _n0$5;
					}
				case 2:
					if (_n0.b.$ === 2) {
						var ms = _n0.a.a;
						var m = _n0.b.a;
						var _n3 = A2(author$project$SvgXY$update, ms, m);
						var a = _n3.a;
						var b = _n3.b;
						return _Utils_Tuple2(
							author$project$SvgControl$CmXY(a),
							b);
					} else {
						break _n0$5;
					}
				case 3:
					if (_n0.b.$ === 3) {
						var ms = _n0.a.a;
						var m = _n0.b.a;
						var _n4 = A2(author$project$SvgLabel$update, ms, m);
						var md = _n4.a;
						var c = _n4.b;
						return _Utils_Tuple2(
							author$project$SvgControl$CmLabel(md),
							c);
					} else {
						break _n0$5;
					}
				default:
					if (_n0.b.$ === 4) {
						var ms = _n0.a.a;
						var m = _n0.b.a;
						var _n5 = A2(author$project$SvgControl$szupdate, ms, m);
						var a = _n5.a;
						var b = _n5.b;
						return _Utils_Tuple2(
							author$project$SvgControl$CmSizer(a),
							b);
					} else {
						break _n0$5;
					}
			}
		}
		return _Utils_Tuple2(model, author$project$SvgCommand$None);
	});
var author$project$SvgControl$update_list = F2(
	function (msgs, model) {
		return A3(
			elm$core$List$foldl,
			F2(
				function (msg, _n0) {
					var mod = _n0.a;
					var cmds = _n0.b;
					var _n1 = A2(author$project$SvgControl$update, msg, mod);
					var modnew = _n1.a;
					var cmd = _n1.b;
					return _Utils_Tuple2(
						modnew,
						A2(elm$core$List$cons, cmd, cmds));
				}),
			_Utils_Tuple2(model, _List_Nil),
			msgs);
	});
var author$project$SvgControlPage$Model = F7(
	function (title, mahrect, srect, spec, control, windowSize, uiTheme) {
		return {l: control, O: mahrect, a8: spec, y: srect, K: title, U: uiTheme, aK: windowSize};
	});
var author$project$SvgThings$Controls = 0;
var author$project$SvgThings$Labels = 1;
var author$project$SvgThings$Pressed = 3;
var author$project$SvgThings$Text = 2;
var author$project$SvgThings$Unpressed = 4;
var author$project$SvgThings$colorFun = F6(
	function (controls, labels, text, pressed, unpressed, uc) {
		switch (uc) {
			case 0:
				return controls;
			case 1:
				return labels;
			case 2:
				return text;
			case 3:
				return pressed;
			default:
				return unpressed;
		}
	});
var author$project$SvgThings$defaultColors = function (uc) {
	switch (uc) {
		case 0:
			return '000000';
		case 1:
			return 'A5A5A5';
		case 2:
			return 'FFFFFF';
		case 3:
			return '0DB00D';
		default:
			return 'C0E4C0';
	}
};
var author$project$SvgThings$toSRect = function (rect) {
	return A4(
		author$project$SvgThings$SRect,
		elm$core$String$fromInt(rect._),
		elm$core$String$fromInt(rect.aa),
		elm$core$String$fromInt(rect.W),
		elm$core$String$fromInt(rect.L));
};
var author$project$Util$RectSize = F2(
	function (width, height) {
		return {M: height, X: width};
	});
var author$project$SvgControlPage$init = F2(
	function (rect, spec) {
		var colors = A5(
			author$project$SvgThings$colorFun,
			A2(
				elm$core$Maybe$withDefault,
				author$project$SvgThings$defaultColors(0),
				spec.af),
			A2(
				elm$core$Maybe$withDefault,
				author$project$SvgThings$defaultColors(1),
				spec.an),
			A2(
				elm$core$Maybe$withDefault,
				author$project$SvgThings$defaultColors(2),
				spec.aG),
			A2(
				elm$core$Maybe$withDefault,
				author$project$SvgThings$defaultColors(3),
				spec.au),
			A2(
				elm$core$Maybe$withDefault,
				author$project$SvgThings$defaultColors(4),
				spec.aI));
		var _n0 = A3(author$project$SvgControl$init, rect, _List_Nil, spec.aB);
		var conmod = _n0.a;
		var cmd = _n0.b;
		var _n1 = A2(
			author$project$SvgControl$update_list,
			A2(elm$core$Maybe$withDefault, _List_Nil, spec.aD),
			conmod);
		var updmod = _n1.a;
		var cmds = _n1.b;
		return _Utils_Tuple2(
			A7(
				author$project$SvgControlPage$Model,
				spec.K,
				rect,
				author$project$SvgThings$toSRect(rect),
				spec,
				updmod,
				A2(author$project$Util$RectSize, 0, 0),
				{aS: colors}),
			author$project$SvgCommand$Batch(
				A2(elm$core$List$cons, cmd, cmds)));
	});
var author$project$SvgLabel$Spec = F2(
	function (name, label) {
		return {q: label, ap: name};
	});
var elm$core$Maybe$andThen = F2(
	function (callback, maybeValue) {
		if (!maybeValue.$) {
			var value = maybeValue.a;
			return callback(value);
		} else {
			return elm$core$Maybe$Nothing;
		}
	});
var elm$core$Maybe$map = F2(
	function (f, maybe) {
		if (!maybe.$) {
			var value = maybe.a;
			return elm$core$Maybe$Just(
				f(value));
		} else {
			return elm$core$Maybe$Nothing;
		}
	});
var author$project$Main$init = function (flags) {
	var wsUrl = A2(
		elm$core$Maybe$withDefault,
		'',
		A2(
			elm$core$Maybe$map,
			function (loc) {
				return 'ws:' + (loc + (':' + elm$core$String$fromInt(flags.Z)));
			},
			A2(
				elm$core$Maybe$andThen,
				elm$core$List$head,
				elm$core$List$tail(
					A2(elm$core$String$split, ':', flags.N)))));
	var rmargin = 4;
	var _n0 = A2(
		author$project$SvgControlPage$init,
		A4(author$project$SvgThings$Rect, 0, 0, flags.X - rmargin, flags.M - rmargin),
		A8(
			author$project$SvgControlPage$Spec,
			wsUrl,
			author$project$SvgControl$CsLabel(
				A2(author$project$SvgLabel$Spec, 'blah', 'no controls loaded!')),
			elm$core$Maybe$Nothing,
			elm$core$Maybe$Nothing,
			elm$core$Maybe$Nothing,
			elm$core$Maybe$Nothing,
			elm$core$Maybe$Nothing,
			elm$core$Maybe$Nothing));
	var sm = _n0.a;
	var cmd = _n0.b;
	return _Utils_Tuple2(
		{i: sm, Y: wsUrl},
		cmd);
};
var elm$json$Json$Decode$value = _Json_decodeValue;
var author$project$Main$receiveTextMetrics = _Platform_incomingPort('receiveTextMetrics', elm$json$Json$Decode$value);
var author$project$Main$WsMsg = function (a) {
	return {$: 0, a: a};
};
var author$project$Main$receiveSocketMsg = _Platform_incomingPort('receiveSocketMsg', elm$json$Json$Decode$value);
var author$project$WebSocket$Data = function (a) {
	return {$: 1, a: a};
};
var author$project$WebSocket$Error = function (a) {
	return {$: 0, a: a};
};
var elm$json$Json$Decode$andThen = _Json_andThen;
var elm$json$Json$Decode$fail = _Json_fail;
var elm$json$Json$Decode$map2 = _Json_map2;
var elm$json$Json$Decode$string = _Json_decodeString;
var author$project$WebSocket$decodeMsg = A2(
	elm$json$Json$Decode$andThen,
	function (msg) {
		switch (msg) {
			case 'error':
				return A3(
					elm$json$Json$Decode$map2,
					F2(
						function (a, b) {
							return author$project$WebSocket$Error(
								{aW: b, ap: a});
						}),
					A2(elm$json$Json$Decode$field, 'name', elm$json$Json$Decode$string),
					A2(elm$json$Json$Decode$field, 'error', elm$json$Json$Decode$string));
			case 'data':
				return A3(
					elm$json$Json$Decode$map2,
					F2(
						function (a, b) {
							return author$project$WebSocket$Data(
								{aU: b, ap: a});
						}),
					A2(elm$json$Json$Decode$field, 'name', elm$json$Json$Decode$string),
					A2(elm$json$Json$Decode$field, 'data', elm$json$Json$Decode$string));
			default:
				var unk = msg;
				return elm$json$Json$Decode$fail('unknown websocketmsg type: ' + unk);
		}
	},
	A2(elm$json$Json$Decode$field, 'msg', elm$json$Json$Decode$string));
var author$project$WebSocket$receive = function (wsmMsg) {
	return function (v) {
		return wsmMsg(
			A2(elm$json$Json$Decode$decodeValue, author$project$WebSocket$decodeMsg, v));
	};
};
var author$project$Main$wsreceive = author$project$Main$receiveSocketMsg(
	author$project$WebSocket$receive(author$project$Main$WsMsg));
var author$project$SvgControlPage$JsonMsg = function (a) {
	return {$: 0, a: a};
};
var author$project$SvgControlPage$Resize = function (a) {
	return {$: 2, a: a};
};
var elm$core$String$fromFloat = _String_fromNumber;
var elm$json$Json$Decode$map = _Json_map1;
var elm$json$Json$Decode$succeed = _Json_succeed;
var elm$virtual_dom$VirtualDom$toHandlerInt = function (handler) {
	switch (handler.$) {
		case 0:
			return 0;
		case 1:
			return 1;
		case 2:
			return 2;
		default:
			return 3;
	}
};
var elm$virtual_dom$VirtualDom$text = _VirtualDom_text;
var elm$svg$Svg$text = elm$virtual_dom$VirtualDom$text;
var elm$svg$Svg$trustedNode = _VirtualDom_nodeNS('http://www.w3.org/2000/svg');
var elm$svg$Svg$text_ = elm$svg$Svg$trustedNode('text');
var elm$svg$Svg$Attributes$dominantBaseline = _VirtualDom_attribute('dominant-baseline');
var elm$svg$Svg$Attributes$fill = _VirtualDom_attribute('fill');
var elm$svg$Svg$Attributes$fontFamily = _VirtualDom_attribute('font-family');
var elm$svg$Svg$Attributes$fontSize = _VirtualDom_attribute('font-size');
var elm$svg$Svg$Attributes$style = _VirtualDom_attribute('style');
var elm$svg$Svg$Attributes$transform = _VirtualDom_attribute('transform');
var lukewestby$elm_template$Template$renderComponent = F3(
	function (record, component, result) {
		if (!component.$) {
			var string = component.a;
			return _Utils_ap(result, string);
		} else {
			var accessor = component.a;
			return _Utils_ap(
				result,
				accessor(record));
		}
	});
var lukewestby$elm_template$Template$render = F2(
	function (record, currentTemplate) {
		return A3(
			elm$core$List$foldr,
			lukewestby$elm_template$Template$renderComponent(record),
			'',
			currentTemplate);
	});
var lukewestby$elm_template$Template$Literal = function (a) {
	return {$: 0, a: a};
};
var lukewestby$elm_template$Template$template = function (initial) {
	return _List_fromArray(
		[
			lukewestby$elm_template$Template$Literal(initial)
		]);
};
var lukewestby$elm_template$Template$withString = F2(
	function (string, currentTemplate) {
		return A2(
			elm$core$List$cons,
			lukewestby$elm_template$Template$Literal(string),
			currentTemplate);
	});
var lukewestby$elm_template$Template$Interpolation = function (a) {
	return {$: 1, a: a};
};
var lukewestby$elm_template$Template$withValue = F2(
	function (interpolator, currentTemplate) {
		return A2(
			elm$core$List$cons,
			lukewestby$elm_template$Template$Interpolation(interpolator),
			currentTemplate);
	});
var author$project$SvgTextSize$calcText = F6(
	function (theme, fontFam, lbtext, labelMeasuredWidth, fontScaling, rect) {
		var yc = rect.aa + (rect.L / 2);
		var yt = yc;
		var xc = rect._ + (rect.W / 2);
		var width = labelMeasuredWidth;
		var tmpl = A2(
			lukewestby$elm_template$Template$withString,
			')',
			A2(
				lukewestby$elm_template$Template$withValue,
				function ($) {
					return $.aN;
				},
				A2(
					lukewestby$elm_template$Template$withString,
					', ',
					A2(
						lukewestby$elm_template$Template$withValue,
						function ($) {
							return $.aL;
						},
						A2(
							lukewestby$elm_template$Template$withString,
							', ',
							A2(
								lukewestby$elm_template$Template$withValue,
								function ($) {
									return $.R;
								},
								A2(
									lukewestby$elm_template$Template$withString,
									', 0, 0, ',
									A2(
										lukewestby$elm_template$Template$withValue,
										function ($) {
											return $.R;
										},
										lukewestby$elm_template$Template$template('matrix(')))))))));
		var scale = fontScaling;
		var xt = xc - ((width * scale) * 0.5);
		var xf = A2(
			lukewestby$elm_template$Template$render,
			{
				R: elm$core$String$fromFloat(scale),
				aL: elm$core$String$fromFloat(xt),
				aN: elm$core$String$fromFloat(yt)
			},
			tmpl);
		return _List_fromArray(
			[
				A2(
				elm$svg$Svg$text_,
				_List_fromArray(
					[
						elm$svg$Svg$Attributes$fill(
						'#' + theme.aS(2)),
						elm$svg$Svg$Attributes$dominantBaseline('middle'),
						elm$svg$Svg$Attributes$fontSize('20px'),
						elm$svg$Svg$Attributes$fontFamily(fontFam),
						elm$svg$Svg$Attributes$transform(xf),
						elm$svg$Svg$Attributes$style('cursor: default;  -webkit-user-select: none;  -moz-user-select: none;  -ms-user-select: none; user-select: none;')
					]),
				_List_fromArray(
					[
						elm$svg$Svg$text(lbtext)
					]))
			]);
	});
var author$project$SvgTextSize$computeFontScaling = F4(
	function (fw, fh, rw, rh) {
		var wr = rw / fw;
		var hr = rh / fh;
		return (_Utils_cmp(wr, hr) < 0) ? wr : hr;
	});
var author$project$SvgTextSize$textHeight = 0.65;
var author$project$SvgTextSize$calcTextSvg = F4(
	function (theme, textString, width20px, rect) {
		var fs = A4(author$project$SvgTextSize$computeFontScaling, width20px, 20.0, rect.W, author$project$SvgTextSize$textHeight * rect.L);
		return A6(author$project$SvgTextSize$calcText, theme, author$project$SvgTextSize$controlFontFamily, textString, width20px, fs, rect);
	});
var author$project$SvgTextSize$onTextSizeReply = F3(
	function (theme, tsr, model) {
		return _Utils_update(
			model,
			{
				T: elm$core$Maybe$Just(tsr.X),
				J: A4(author$project$SvgTextSize$calcTextSvg, theme, model.q, tsr.X, model.ay)
			});
	});
var author$project$SvgControl$onTextSize = F3(
	function (theme, tsr, model) {
		switch (model.$) {
			case 0:
				var m = model.a;
				return author$project$SvgControl$CmButton(
					A3(author$project$SvgTextSize$onTextSizeReply, theme, tsr, m));
			case 1:
				var m = model.a;
				return author$project$SvgControl$CmSlider(
					A3(author$project$SvgTextSize$onTextSizeReply, theme, tsr, m));
			case 2:
				var m = model.a;
				return author$project$SvgControl$CmXY(
					A3(author$project$SvgTextSize$onTextSizeReply, theme, tsr, m));
			case 3:
				var m = model.a;
				return author$project$SvgControl$CmLabel(
					A3(author$project$SvgTextSize$onTextSizeReply, theme, tsr, m));
			default:
				var m = model.a;
				return author$project$SvgControl$CmSizer(
					A3(author$project$SvgControl$szOnTextSize, theme, tsr, m));
		}
	});
var author$project$SvgControl$szOnTextSize = F3(
	function (theme, tsr, model) {
		var _n0 = tsr.ae;
		if (_n0.b) {
			var idx = _n0.a;
			var rst = _n0.b;
			var _n1 = A2(elm$core$Dict$get, idx, model.d);
			if (!_n1.$) {
				var control = _n1.a;
				var nc = A3(
					author$project$SvgControl$onTextSize,
					theme,
					_Utils_update(
						tsr,
						{ae: rst}),
					control);
				return _Utils_update(
					model,
					{
						d: A3(elm$core$Dict$insert, idx, nc, model.d)
					});
			} else {
				return model;
			}
		} else {
			return model;
		}
	});
var author$project$SvgControlPage$onTextSize = F2(
	function (tsr, model) {
		return _Utils_update(
			model,
			{
				l: A3(author$project$SvgControl$onTextSize, model.U, tsr, model.l)
			});
	});
var author$project$SvgButton$SvgUpdate = function (a) {
	return {$: 5, a: a};
};
var author$project$SvgButton$jsUpdateType = function (ut) {
	switch (ut) {
		case 'Press':
			return elm$json$Json$Decode$succeed(0);
		case 'Unpress':
			return elm$json$Json$Decode$succeed(1);
		default:
			return elm$json$Json$Decode$succeed(1);
	}
};
var elm$json$Json$Decode$list = _Json_decodeList;
var author$project$SvgThings$decodeControlId = elm$json$Json$Decode$list(elm$json$Json$Decode$int);
var elm$json$Json$Decode$oneOf = _Json_oneOf;
var elm$json$Json$Decode$maybe = function (decoder) {
	return elm$json$Json$Decode$oneOf(
		_List_fromArray(
			[
				A2(elm$json$Json$Decode$map, elm$core$Maybe$Just, decoder),
				elm$json$Json$Decode$succeed(elm$core$Maybe$Nothing)
			]));
};
var author$project$SvgButton$jsUpdateMessage = A4(
	elm$json$Json$Decode$map3,
	author$project$SvgButton$UpdateMessage,
	A2(elm$json$Json$Decode$field, 'controlId', author$project$SvgThings$decodeControlId),
	elm$json$Json$Decode$maybe(
		A2(
			elm$json$Json$Decode$andThen,
			author$project$SvgButton$jsUpdateType,
			A2(elm$json$Json$Decode$field, 'state', elm$json$Json$Decode$string))),
	elm$json$Json$Decode$maybe(
		A2(elm$json$Json$Decode$field, 'label', elm$json$Json$Decode$string)));
var author$project$SvgControl$CaButton = function (a) {
	return {$: 0, a: a};
};
var author$project$SvgControl$CaLabel = function (a) {
	return {$: 3, a: a};
};
var author$project$SvgControl$CaSlider = function (a) {
	return {$: 1, a: a};
};
var author$project$SvgControl$CaXY = function (a) {
	return {$: 2, a: a};
};
var author$project$SvgControl$CaSizer = function (a) {
	return {$: 4, a: a};
};
var author$project$SvgControl$SzCMsg = F2(
	function (a, b) {
		return {$: 0, a: a, b: b};
	});
var author$project$SvgControl$myTail = function (lst) {
	var tl = elm$core$List$tail(lst);
	if (!tl.$) {
		var l = tl.a;
		return l;
	} else {
		return _List_Nil;
	}
};
var author$project$SvgControl$toCtrlMsg = F2(
	function (id, msg) {
		var _n0 = elm$core$List$head(id);
		if (_n0.$ === 1) {
			return msg;
		} else {
			var x = _n0.a;
			return author$project$SvgControl$CaSizer(
				A2(
					author$project$SvgControl$SzCMsg,
					x,
					A2(
						author$project$SvgControl$toCtrlMsg,
						author$project$SvgControl$myTail(id),
						msg)));
		}
	});
var author$project$SvgLabel$SvgUpdate = function (a) {
	return {$: 0, a: a};
};
var author$project$SvgLabel$UpdateMessage = F2(
	function (controlId, label) {
		return {ae: controlId, q: label};
	});
var author$project$SvgLabel$jsUpdateMessage = A3(
	elm$json$Json$Decode$map2,
	author$project$SvgLabel$UpdateMessage,
	A2(elm$json$Json$Decode$field, 'controlId', author$project$SvgThings$decodeControlId),
	A2(elm$json$Json$Decode$field, 'label', elm$json$Json$Decode$string));
var author$project$SvgSlider$SvgUpdate = function (a) {
	return {$: 6, a: a};
};
var author$project$SvgSlider$jsUpdateType = function (ut) {
	switch (ut) {
		case 'Press':
			return elm$json$Json$Decode$succeed(0);
		case 'Unpress':
			return elm$json$Json$Decode$succeed(1);
		default:
			return elm$json$Json$Decode$succeed(1);
	}
};
var elm$json$Json$Decode$map4 = _Json_map4;
var author$project$SvgSlider$jsUpdateMessage = A5(
	elm$json$Json$Decode$map4,
	author$project$SvgSlider$UpdateMessage,
	A2(elm$json$Json$Decode$field, 'controlId', author$project$SvgThings$decodeControlId),
	elm$json$Json$Decode$maybe(
		A2(
			elm$json$Json$Decode$andThen,
			author$project$SvgSlider$jsUpdateType,
			A2(elm$json$Json$Decode$field, 'state', elm$json$Json$Decode$string))),
	elm$json$Json$Decode$maybe(
		A2(elm$json$Json$Decode$field, 'location', elm$json$Json$Decode$float)),
	elm$json$Json$Decode$maybe(
		A2(elm$json$Json$Decode$field, 'label', elm$json$Json$Decode$string)));
var author$project$SvgXY$SvgUpdate = function (a) {
	return {$: 6, a: a};
};
var author$project$SvgXY$jsUpdateType = function (ut) {
	switch (ut) {
		case 'Press':
			return elm$json$Json$Decode$succeed(0);
		case 'Unpress':
			return elm$json$Json$Decode$succeed(1);
		default:
			return elm$json$Json$Decode$succeed(1);
	}
};
var author$project$SvgXY$jsUpdateMessage = A5(
	elm$json$Json$Decode$map4,
	author$project$SvgXY$UpdateMessage,
	A2(elm$json$Json$Decode$field, 'controlId', author$project$SvgThings$decodeControlId),
	elm$json$Json$Decode$maybe(
		A2(
			elm$json$Json$Decode$andThen,
			author$project$SvgXY$jsUpdateType,
			A2(elm$json$Json$Decode$field, 'state', elm$json$Json$Decode$string))),
	elm$json$Json$Decode$maybe(
		A2(
			elm$json$Json$Decode$field,
			'location',
			A2(
				elm$json$Json$Decode$andThen,
				function (lst) {
					if ((lst.b && lst.b.b) && (!lst.b.b.b)) {
						var a = lst.a;
						var _n1 = lst.b;
						var b = _n1.a;
						return elm$json$Json$Decode$succeed(
							_Utils_Tuple2(a, b));
					} else {
						return elm$json$Json$Decode$fail('location requires exactly two elements, [x,y]');
					}
				},
				elm$json$Json$Decode$list(elm$json$Json$Decode$float)))),
	elm$json$Json$Decode$maybe(
		A2(elm$json$Json$Decode$field, 'label', elm$json$Json$Decode$string)));
var author$project$SvgControl$jsUmType = function (wat) {
	switch (wat) {
		case 'button':
			return A2(
				elm$json$Json$Decode$andThen,
				function (x) {
					return elm$json$Json$Decode$succeed(
						A2(
							author$project$SvgControl$toCtrlMsg,
							x.ae,
							author$project$SvgControl$CaButton(
								author$project$SvgButton$SvgUpdate(x))));
				},
				author$project$SvgButton$jsUpdateMessage);
		case 'slider':
			return A2(
				elm$json$Json$Decode$andThen,
				function (x) {
					return elm$json$Json$Decode$succeed(
						A2(
							author$project$SvgControl$toCtrlMsg,
							x.ae,
							author$project$SvgControl$CaSlider(
								author$project$SvgSlider$SvgUpdate(x))));
				},
				author$project$SvgSlider$jsUpdateMessage);
		case 'xy':
			return A2(
				elm$json$Json$Decode$andThen,
				function (x) {
					return elm$json$Json$Decode$succeed(
						A2(
							author$project$SvgControl$toCtrlMsg,
							x.ae,
							author$project$SvgControl$CaXY(
								author$project$SvgXY$SvgUpdate(x))));
				},
				author$project$SvgXY$jsUpdateMessage);
		case 'label':
			return A2(
				elm$json$Json$Decode$andThen,
				function (x) {
					return elm$json$Json$Decode$succeed(
						A2(
							author$project$SvgControl$toCtrlMsg,
							x.ae,
							author$project$SvgControl$CaLabel(
								author$project$SvgLabel$SvgUpdate(x))));
				},
				author$project$SvgLabel$jsUpdateMessage);
		default:
			return elm$json$Json$Decode$fail('unknown update type' + wat);
	}
};
var author$project$SvgControl$jsUpdateMessage = A2(
	elm$json$Json$Decode$andThen,
	author$project$SvgControl$jsUmType,
	A2(elm$json$Json$Decode$field, 'controlType', elm$json$Json$Decode$string));
var author$project$SvgControlPage$CMsg = function (a) {
	return {$: 1, a: a};
};
var author$project$SvgControlPage$JmSpec = function (a) {
	return {$: 0, a: a};
};
var author$project$SvgControlPage$JmUpdate = function (a) {
	return {$: 1, a: a};
};
var author$project$SvgButton$Spec = F2(
	function (name, label) {
		return {q: label, ap: name};
	});
var author$project$SvgButton$jsSpec = A3(
	elm$json$Json$Decode$map2,
	author$project$SvgButton$Spec,
	A2(elm$json$Json$Decode$field, 'name', elm$json$Json$Decode$string),
	elm$json$Json$Decode$maybe(
		A2(elm$json$Json$Decode$field, 'label', elm$json$Json$Decode$string)));
var author$project$SvgControl$CsButton = function (a) {
	return {$: 0, a: a};
};
var author$project$SvgControl$CsSizer = function (a) {
	return {$: 4, a: a};
};
var author$project$SvgControl$CsSlider = function (a) {
	return {$: 1, a: a};
};
var author$project$SvgControl$CsXY = function (a) {
	return {$: 2, a: a};
};
var author$project$SvgControl$SzSpec = F3(
	function (orientation, proportions, controls) {
		return {d: controls, C: orientation, D: proportions};
	});
var author$project$SvgControl$processProps = function (lst) {
	var s = elm$core$List$sum(lst);
	return A2(
		elm$core$List$map,
		function (x) {
			return x / s;
		},
		lst);
};
var author$project$SvgLabel$jsSpec = A3(
	elm$json$Json$Decode$map2,
	author$project$SvgLabel$Spec,
	A2(elm$json$Json$Decode$field, 'name', elm$json$Json$Decode$string),
	A2(elm$json$Json$Decode$field, 'label', elm$json$Json$Decode$string));
var author$project$SvgSlider$Spec = F3(
	function (name, label, orientation) {
		return {q: label, ap: name, C: orientation};
	});
var author$project$SvgThings$Horizontal = 1;
var author$project$SvgThings$Vertical = 0;
var author$project$SvgThings$jsOrientation = function (o) {
	switch (o) {
		case 'vertical':
			return elm$json$Json$Decode$succeed(0);
		case 'horizontal':
			return elm$json$Json$Decode$succeed(1);
		default:
			return elm$json$Json$Decode$succeed(1);
	}
};
var author$project$SvgSlider$jsSpec = A4(
	elm$json$Json$Decode$map3,
	author$project$SvgSlider$Spec,
	A2(elm$json$Json$Decode$field, 'name', elm$json$Json$Decode$string),
	elm$json$Json$Decode$maybe(
		A2(elm$json$Json$Decode$field, 'label', elm$json$Json$Decode$string)),
	A2(
		elm$json$Json$Decode$andThen,
		author$project$SvgThings$jsOrientation,
		A2(elm$json$Json$Decode$field, 'orientation', elm$json$Json$Decode$string)));
var author$project$SvgXY$Spec = F2(
	function (name, label) {
		return {q: label, ap: name};
	});
var author$project$SvgXY$jsSpec = A3(
	elm$json$Json$Decode$map2,
	author$project$SvgXY$Spec,
	A2(elm$json$Json$Decode$field, 'name', elm$json$Json$Decode$string),
	elm$json$Json$Decode$maybe(
		A2(elm$json$Json$Decode$field, 'label', elm$json$Json$Decode$string)));
var elm$json$Json$Decode$lazy = function (thunk) {
	return A2(
		elm$json$Json$Decode$andThen,
		thunk,
		elm$json$Json$Decode$succeed(0));
};
var author$project$SvgControl$jsCs = function (t) {
	switch (t) {
		case 'button':
			return A2(
				elm$json$Json$Decode$andThen,
				function (a) {
					return elm$json$Json$Decode$succeed(
						author$project$SvgControl$CsButton(a));
				},
				author$project$SvgButton$jsSpec);
		case 'slider':
			return A2(
				elm$json$Json$Decode$andThen,
				function (a) {
					return elm$json$Json$Decode$succeed(
						author$project$SvgControl$CsSlider(a));
				},
				author$project$SvgSlider$jsSpec);
		case 'xy':
			return A2(
				elm$json$Json$Decode$andThen,
				function (a) {
					return elm$json$Json$Decode$succeed(
						author$project$SvgControl$CsXY(a));
				},
				author$project$SvgXY$jsSpec);
		case 'label':
			return A2(
				elm$json$Json$Decode$andThen,
				function (a) {
					return elm$json$Json$Decode$succeed(
						author$project$SvgControl$CsLabel(a));
				},
				author$project$SvgLabel$jsSpec);
		case 'sizer':
			return A2(
				elm$json$Json$Decode$andThen,
				function (a) {
					return elm$json$Json$Decode$succeed(
						author$project$SvgControl$CsSizer(a));
				},
				author$project$SvgControl$cyclic$jsSzSpec());
		default:
			return elm$json$Json$Decode$fail('unkown type: ' + t);
	}
};
function author$project$SvgControl$cyclic$jsSpec() {
	return A2(
		elm$json$Json$Decode$andThen,
		author$project$SvgControl$jsCs,
		A2(elm$json$Json$Decode$field, 'type', elm$json$Json$Decode$string));
}
function author$project$SvgControl$cyclic$jsSzSpec() {
	return A4(
		elm$json$Json$Decode$map3,
		author$project$SvgControl$SzSpec,
		A2(
			elm$json$Json$Decode$andThen,
			author$project$SvgThings$jsOrientation,
			A2(elm$json$Json$Decode$field, 'orientation', elm$json$Json$Decode$string)),
		A2(
			elm$json$Json$Decode$andThen,
			function (x) {
				return elm$json$Json$Decode$succeed(
					A2(elm$core$Maybe$map, author$project$SvgControl$processProps, x));
			},
			elm$json$Json$Decode$maybe(
				A2(
					elm$json$Json$Decode$field,
					'proportions',
					elm$json$Json$Decode$list(elm$json$Json$Decode$float)))),
		A2(
			elm$json$Json$Decode$field,
			'controls',
			elm$json$Json$Decode$list(
				elm$json$Json$Decode$lazy(
					function (_n0) {
						return author$project$SvgControl$cyclic$jsSpec();
					}))));
}
var author$project$SvgControl$jsSpec = author$project$SvgControl$cyclic$jsSpec();
author$project$SvgControl$cyclic$jsSpec = function () {
	return author$project$SvgControl$jsSpec;
};
var author$project$SvgControl$jsSzSpec = author$project$SvgControl$cyclic$jsSzSpec();
author$project$SvgControl$cyclic$jsSzSpec = function () {
	return author$project$SvgControl$jsSzSpec;
};
var elm$json$Json$Decode$map8 = _Json_map8;
var author$project$SvgControlPage$jsSpec = A9(
	elm$json$Json$Decode$map8,
	author$project$SvgControlPage$Spec,
	A2(elm$json$Json$Decode$field, 'title', elm$json$Json$Decode$string),
	A2(elm$json$Json$Decode$field, 'rootControl', author$project$SvgControl$jsSpec),
	elm$json$Json$Decode$maybe(
		A2(
			elm$json$Json$Decode$field,
			'state',
			elm$json$Json$Decode$list(author$project$SvgControl$jsUpdateMessage))),
	elm$json$Json$Decode$maybe(
		A2(elm$json$Json$Decode$field, 'controlsColor', elm$json$Json$Decode$string)),
	elm$json$Json$Decode$maybe(
		A2(elm$json$Json$Decode$field, 'labelsColor', elm$json$Json$Decode$string)),
	elm$json$Json$Decode$maybe(
		A2(elm$json$Json$Decode$field, 'textColor', elm$json$Json$Decode$string)),
	elm$json$Json$Decode$maybe(
		A2(elm$json$Json$Decode$field, 'pressedColor', elm$json$Json$Decode$string)),
	elm$json$Json$Decode$maybe(
		A2(elm$json$Json$Decode$field, 'unpressedColor', elm$json$Json$Decode$string)));
var author$project$SvgControlPage$jsMessage = elm$json$Json$Decode$oneOf(
	_List_fromArray(
		[
			A2(
			elm$json$Json$Decode$andThen,
			function (x) {
				return elm$json$Json$Decode$succeed(
					author$project$SvgControlPage$JmSpec(x));
			},
			author$project$SvgControlPage$jsSpec),
			A2(
			elm$json$Json$Decode$andThen,
			function (x) {
				return elm$json$Json$Decode$succeed(
					author$project$SvgControlPage$JmUpdate(
						author$project$SvgControlPage$CMsg(x)));
			},
			author$project$SvgControl$jsUpdateMessage)
		]));
var author$project$SvgButton$resize = F2(
	function (model, rect) {
		var newmodel = _Utils_update(
			model,
			{
				ay: rect,
				y: A4(
					author$project$SvgThings$SRect,
					elm$core$String$fromInt(rect._),
					elm$core$String$fromInt(rect.aa),
					elm$core$String$fromInt(rect.W),
					elm$core$String$fromInt(rect.L)),
				T: elm$core$Maybe$Nothing,
				J: _List_Nil
			});
		return _Utils_Tuple2(
			newmodel,
			author$project$SvgTextSize$resizeCommand(newmodel));
	});
var author$project$SvgLabel$resize = F2(
	function (model, rect) {
		var newmodel = _Utils_update(
			model,
			{
				ay: rect,
				y: A4(
					author$project$SvgThings$SRect,
					elm$core$String$fromInt(rect._),
					elm$core$String$fromInt(rect.aa),
					elm$core$String$fromInt(rect.W),
					elm$core$String$fromInt(rect.L)),
				T: elm$core$Maybe$Nothing,
				J: _List_Nil
			});
		return _Utils_Tuple2(
			newmodel,
			author$project$SvgTextSize$resizeCommand(newmodel));
	});
var author$project$SvgSlider$resize = F2(
	function (model, rect) {
		var newmodel = _Utils_update(
			model,
			{
				ay: rect,
				y: A4(
					author$project$SvgThings$SRect,
					elm$core$String$fromInt(rect._),
					elm$core$String$fromInt(rect.aa),
					elm$core$String$fromInt(rect.W),
					elm$core$String$fromInt(rect.L)),
				T: elm$core$Maybe$Nothing,
				J: _List_Nil
			});
		return _Utils_Tuple2(
			newmodel,
			author$project$SvgTextSize$resizeCommand(newmodel));
	});
var author$project$SvgXY$resize = F2(
	function (model, rect) {
		var newmodel = _Utils_update(
			model,
			{
				ay: rect,
				y: A4(
					author$project$SvgThings$SRect,
					elm$core$String$fromInt(rect._),
					elm$core$String$fromInt(rect.aa),
					elm$core$String$fromInt(rect.W),
					elm$core$String$fromInt(rect.L)),
				T: elm$core$Maybe$Nothing,
				J: _List_Nil
			});
		return _Utils_Tuple2(
			newmodel,
			author$project$SvgTextSize$resizeCommand(newmodel));
	});
var author$project$SvgControl$resize = F2(
	function (model, rect) {
		var aptg = F2(
			function (f, _n7) {
				var m = _n7.a;
				var c = _n7.b;
				return _Utils_Tuple2(
					f(m),
					c);
			});
		switch (model.$) {
			case 0:
				var mod = model.a;
				return A2(
					aptg,
					author$project$SvgControl$CmButton,
					A2(
						author$project$SvgButton$resize,
						mod,
						A2(author$project$SvgThings$shrinkRect, author$project$SvgControl$border, rect)));
			case 1:
				var mod = model.a;
				return A2(
					aptg,
					author$project$SvgControl$CmSlider,
					A2(
						author$project$SvgSlider$resize,
						mod,
						A2(author$project$SvgThings$shrinkRect, author$project$SvgControl$border, rect)));
			case 2:
				var mod = model.a;
				return A2(
					aptg,
					author$project$SvgControl$CmXY,
					A2(
						author$project$SvgXY$resize,
						mod,
						A2(author$project$SvgThings$shrinkRect, author$project$SvgControl$border, rect)));
			case 3:
				var mod = model.a;
				return A2(
					aptg,
					author$project$SvgControl$CmLabel,
					A2(
						author$project$SvgLabel$resize,
						mod,
						A2(author$project$SvgThings$shrinkRect, author$project$SvgControl$border, rect)));
			default:
				var mod = model.a;
				return A2(
					aptg,
					author$project$SvgControl$CmSizer,
					A2(author$project$SvgControl$szresize, mod, rect));
		}
	});
var author$project$SvgControl$szresize = F2(
	function (model, rect) {
		var clist = elm$core$Dict$toList(model.d);
		var rlist = A4(
			author$project$SvgControl$mkRlist,
			model.C,
			rect,
			elm$core$List$length(clist),
			model.D);
		var rlist2 = A2(
			elm$core$List$map,
			function (_n4) {
				var _n5 = _n4.a;
				var i = _n5.a;
				var c = _n5.b;
				var r = _n4.b;
				return _Utils_Tuple2(
					i,
					A2(author$project$SvgControl$resize, c, r));
			},
			A2(author$project$SvgControl$zip, clist, rlist));
		var cmds = A2(
			elm$core$List$map,
			function (_n2) {
				var i = _n2.a;
				var _n3 = _n2.b;
				var m = _n3.a;
				var c = _n3.b;
				return c;
			},
			rlist2);
		var controls = A2(
			elm$core$List$map,
			function (_n0) {
				var i = _n0.a;
				var _n1 = _n0.b;
				var m = _n1.a;
				var c = _n1.b;
				return _Utils_Tuple2(i, m);
			},
			rlist2);
		var cdict = elm$core$Dict$fromList(controls);
		var nm = _Utils_update(
			model,
			{d: cdict, ay: rect});
		return _Utils_Tuple2(
			nm,
			author$project$SvgCommand$Batch(cmds));
	});
var author$project$SvgControlPage$resize = F2(
	function (newSize, model) {
		var nr = A4(
			author$project$SvgThings$Rect,
			0,
			0,
			elm$core$Basics$round(newSize.X - 1),
			elm$core$Basics$round(newSize.M - 4));
		var _n0 = A2(author$project$SvgControl$resize, model.l, nr);
		var ctrl = _n0.a;
		var cmd = _n0.b;
		return _Utils_Tuple2(
			_Utils_update(
				model,
				{
					l: ctrl,
					O: nr,
					y: author$project$SvgThings$toSRect(nr),
					aK: newSize
				}),
			cmd);
	});
var elm$json$Json$Decode$decodeString = _Json_runOnString;
var author$project$SvgControlPage$update = F2(
	function (msg, model) {
		update:
		while (true) {
			switch (msg.$) {
				case 0:
					var s = msg.a;
					var _n1 = A2(elm$json$Json$Decode$decodeString, author$project$SvgControlPage$jsMessage, s);
					if (!_n1.$) {
						if (!_n1.a.$) {
							var spec = _n1.a.a;
							return A2(author$project$SvgControlPage$init, model.O, spec);
						} else {
							var jmact = _n1.a.a;
							var $temp$msg = jmact,
								$temp$model = model;
							msg = $temp$msg;
							model = $temp$model;
							continue update;
						}
					} else {
						var e = _n1.a;
						return _Utils_Tuple2(
							_Utils_update(
								model,
								{
									K: elm$json$Json$Decode$errorToString(e)
								}),
							author$project$SvgCommand$None);
					}
				case 1:
					var act = msg.a;
					var wha = A2(author$project$SvgControl$update, act, model.l);
					var newmod = _Utils_update(
						model,
						{l: wha.a});
					return _Utils_Tuple2(newmod, wha.b);
				case 2:
					var newSize = msg.a;
					return A2(author$project$SvgControlPage$resize, newSize, model);
				default:
					return _Utils_Tuple2(model, author$project$SvgCommand$None);
			}
		}
	});
var author$project$SvgButton$NoOp = {$: 2};
var author$project$SvgButton$SvgPress = {$: 0};
var author$project$SvgButton$SvgUnpress = {$: 1};
var author$project$SvgButton$SvgTouch = function (a) {
	return {$: 4, a: a};
};
var elm$virtual_dom$VirtualDom$Custom = function (a) {
	return {$: 3, a: a};
};
var elm$virtual_dom$VirtualDom$on = _VirtualDom_on;
var author$project$SvgButton$buttonEvt = F2(
	function (evtname, mkmsg) {
		return A2(
			elm$virtual_dom$VirtualDom$on,
			evtname,
			elm$virtual_dom$VirtualDom$Custom(
				A2(
					elm$json$Json$Decode$map,
					function (v) {
						return {
							a1: mkmsg(v),
							a5: true,
							a9: true
						};
					},
					elm$json$Json$Decode$value)));
	});
var author$project$SvgTouch$SvgTouchCancel = function (a) {
	return {$: 3, a: a};
};
var author$project$SvgButton$onTouchCancel = A2(
	author$project$SvgButton$buttonEvt,
	'touchcancel',
	function (e) {
		return author$project$SvgButton$SvgTouch(
			author$project$SvgTouch$SvgTouchCancel(e));
	});
var author$project$SvgTouch$SvgTouchEnd = function (a) {
	return {$: 2, a: a};
};
var author$project$SvgButton$onTouchEnd = A2(
	author$project$SvgButton$buttonEvt,
	'touchend',
	function (e) {
		return author$project$SvgButton$SvgTouch(
			author$project$SvgTouch$SvgTouchEnd(e));
	});
var author$project$SvgTouch$SvgTouchLeave = function (a) {
	return {$: 4, a: a};
};
var author$project$SvgButton$onTouchLeave = A2(
	author$project$SvgButton$buttonEvt,
	'touchleave',
	function (e) {
		return author$project$SvgButton$SvgTouch(
			author$project$SvgTouch$SvgTouchLeave(e));
	});
var author$project$SvgTouch$SvgTouchMove = function (a) {
	return {$: 1, a: a};
};
var author$project$SvgButton$onTouchMove = A2(
	author$project$SvgButton$buttonEvt,
	'touchmove',
	function (e) {
		return author$project$SvgButton$SvgTouch(
			author$project$SvgTouch$SvgTouchMove(e));
	});
var author$project$SvgTouch$SvgTouchStart = function (a) {
	return {$: 0, a: a};
};
var author$project$SvgButton$onTouchStart = A2(
	author$project$SvgButton$buttonEvt,
	'touchstart',
	function (e) {
		return author$project$SvgButton$SvgTouch(
			author$project$SvgTouch$SvgTouchStart(e));
	});
var elm$virtual_dom$VirtualDom$Normal = function (a) {
	return {$: 0, a: a};
};
var elm$html$Html$Events$on = F2(
	function (event, decoder) {
		return A2(
			elm$virtual_dom$VirtualDom$on,
			event,
			elm$virtual_dom$VirtualDom$Normal(decoder));
	});
var elm$html$Html$Events$onMouseDown = function (msg) {
	return A2(
		elm$html$Html$Events$on,
		'mousedown',
		elm$json$Json$Decode$succeed(msg));
};
var elm$html$Html$Events$onMouseOut = function (msg) {
	return A2(
		elm$html$Html$Events$on,
		'mouseout',
		elm$json$Json$Decode$succeed(msg));
};
var elm$html$Html$Events$onMouseUp = function (msg) {
	return A2(
		elm$html$Html$Events$on,
		'mouseup',
		elm$json$Json$Decode$succeed(msg));
};
var author$project$SvgButton$buildEvtHandlerList = function (touchonly) {
	var te = _List_fromArray(
		[author$project$SvgButton$onTouchStart, author$project$SvgButton$onTouchEnd, author$project$SvgButton$onTouchCancel, author$project$SvgButton$onTouchLeave, author$project$SvgButton$onTouchMove]);
	var me = _List_fromArray(
		[
			elm$html$Html$Events$onMouseDown(author$project$SvgButton$SvgPress),
			elm$html$Html$Events$onMouseUp(author$project$SvgButton$SvgUnpress),
			elm$html$Html$Events$onMouseOut(author$project$SvgButton$SvgUnpress)
		]);
	return touchonly ? te : A2(elm$core$List$append, me, te);
};
var elm$svg$Svg$g = elm$svg$Svg$trustedNode('g');
var elm$svg$Svg$rect = elm$svg$Svg$trustedNode('rect');
var elm$svg$Svg$Attributes$height = _VirtualDom_attribute('height');
var elm$svg$Svg$Attributes$rx = _VirtualDom_attribute('rx');
var elm$svg$Svg$Attributes$ry = _VirtualDom_attribute('ry');
var elm$svg$Svg$Attributes$width = _VirtualDom_attribute('width');
var elm$svg$Svg$Attributes$x = _VirtualDom_attribute('x');
var elm$svg$Svg$Attributes$y = _VirtualDom_attribute('y');
var elm$virtual_dom$VirtualDom$map = _VirtualDom_map;
var author$project$SvgButton$view = F2(
	function (theme, model) {
		return A2(
			elm$svg$Svg$g,
			author$project$SvgButton$buildEvtHandlerList(model.aH),
			_List_fromArray(
				[
					A2(
					elm$svg$Svg$rect,
					_List_fromArray(
						[
							elm$svg$Svg$Attributes$x(model.y._),
							elm$svg$Svg$Attributes$y(model.y.aa),
							elm$svg$Svg$Attributes$width(model.y.W),
							elm$svg$Svg$Attributes$height(model.y.L),
							elm$svg$Svg$Attributes$rx('15'),
							elm$svg$Svg$Attributes$ry('15'),
							elm$svg$Svg$Attributes$style(
							'fill: #' + (theme.aS(
								model.n ? 3 : 4) + ';'))
						]),
					_List_Nil),
					A2(
					elm$virtual_dom$VirtualDom$map,
					function (_n0) {
						return author$project$SvgButton$NoOp;
					},
					A2(elm$svg$Svg$g, _List_Nil, model.J))
				]));
	});
var author$project$SvgLabel$NoOp = {$: 1};
var author$project$SvgLabel$view = F2(
	function (theme, model) {
		var lbrect = A2(
			elm$svg$Svg$rect,
			_List_fromArray(
				[
					elm$svg$Svg$Attributes$x(model.y._),
					elm$svg$Svg$Attributes$y(model.y.aa),
					elm$svg$Svg$Attributes$width(model.y.W),
					elm$svg$Svg$Attributes$height(model.y.L),
					elm$svg$Svg$Attributes$rx('15'),
					elm$svg$Svg$Attributes$ry('15'),
					elm$svg$Svg$Attributes$style(
					'fill: #' + (theme.aS(1) + ';'))
				]),
			_List_Nil);
		var svgl = A2(elm$core$List$cons, lbrect, model.J);
		return A2(
			elm$virtual_dom$VirtualDom$map,
			function (_n0) {
				return author$project$SvgLabel$NoOp;
			},
			A2(elm$svg$Svg$g, _List_Nil, svgl));
	});
var author$project$SvgSlider$NoOp = {$: 2};
var author$project$SvgSlider$SvgPress = function (a) {
	return {$: 0, a: a};
};
var author$project$SvgSlider$sliderEvt = F2(
	function (evtname, mkmsg) {
		return A2(
			elm$virtual_dom$VirtualDom$on,
			evtname,
			elm$virtual_dom$VirtualDom$Custom(
				A2(
					elm$json$Json$Decode$map,
					function (v) {
						return {
							a1: mkmsg(v),
							a5: true,
							a9: true
						};
					},
					elm$json$Json$Decode$value)));
	});
var author$project$SvgSlider$onMouseDown = A2(author$project$SvgSlider$sliderEvt, 'mousedown', author$project$SvgSlider$SvgPress);
var author$project$SvgSlider$SvgUnpress = function (a) {
	return {$: 1, a: a};
};
var author$project$SvgSlider$onMouseLeave = A2(author$project$SvgSlider$sliderEvt, 'mouseleave', author$project$SvgSlider$SvgUnpress);
var author$project$SvgSlider$SvgMoved = function (a) {
	return {$: 4, a: a};
};
var author$project$SvgSlider$onMouseMove = A2(author$project$SvgSlider$sliderEvt, 'mousemove', author$project$SvgSlider$SvgMoved);
var author$project$SvgSlider$onMouseUp = A2(author$project$SvgSlider$sliderEvt, 'mouseup', author$project$SvgSlider$SvgUnpress);
var author$project$SvgSlider$SvgTouch = function (a) {
	return {$: 5, a: a};
};
var author$project$SvgSlider$onTouchCancel = A2(
	author$project$SvgSlider$sliderEvt,
	'touchcancel',
	function (e) {
		return author$project$SvgSlider$SvgTouch(
			author$project$SvgTouch$SvgTouchCancel(e));
	});
var author$project$SvgSlider$onTouchEnd = A2(
	author$project$SvgSlider$sliderEvt,
	'touchend',
	function (e) {
		return author$project$SvgSlider$SvgTouch(
			author$project$SvgTouch$SvgTouchEnd(e));
	});
var author$project$SvgSlider$onTouchLeave = A2(
	author$project$SvgSlider$sliderEvt,
	'touchleave',
	function (e) {
		return author$project$SvgSlider$SvgTouch(
			author$project$SvgTouch$SvgTouchLeave(e));
	});
var author$project$SvgSlider$onTouchMove = A2(
	author$project$SvgSlider$sliderEvt,
	'touchmove',
	function (e) {
		return author$project$SvgSlider$SvgTouch(
			author$project$SvgTouch$SvgTouchMove(e));
	});
var author$project$SvgSlider$onTouchStart = A2(
	author$project$SvgSlider$sliderEvt,
	'touchstart',
	function (e) {
		return author$project$SvgSlider$SvgTouch(
			author$project$SvgTouch$SvgTouchStart(e));
	});
var author$project$SvgSlider$buildEvtHandlerList = function (touchonly) {
	var te = _List_fromArray(
		[author$project$SvgSlider$onTouchStart, author$project$SvgSlider$onTouchEnd, author$project$SvgSlider$onTouchCancel, author$project$SvgSlider$onTouchLeave, author$project$SvgSlider$onTouchMove]);
	var me = _List_fromArray(
		[author$project$SvgSlider$onMouseDown, author$project$SvgSlider$onMouseUp, author$project$SvgSlider$onMouseLeave, author$project$SvgSlider$onMouseMove]);
	return touchonly ? te : A2(elm$core$List$append, me, te);
};
var bburdette$toop$Toop$T4 = F4(
	function (a, b, c, d) {
		return {$: 0, a: a, b: b, c: c, d: d};
	});
var author$project$SvgSlider$view = F2(
	function (theme, model) {
		var evtlist = author$project$SvgSlider$buildEvtHandlerList(model.aH);
		var _n0 = function () {
			var _n1 = model.C;
			if (!_n1) {
				return A4(
					bburdette$toop$Toop$T4,
					model.y._,
					elm$core$String$fromInt(
						elm$core$Basics$round(model.N * model.ay.L) + model.ay.aa),
					model.y.W,
					'3');
			} else {
				return A4(
					bburdette$toop$Toop$T4,
					elm$core$String$fromInt(
						elm$core$Basics$round(model.N * model.ay.W) + model.ay._),
					model.y.aa,
					'3',
					model.y.L);
			}
		}();
		var sx = _n0.a;
		var sy = _n0.b;
		var sw = _n0.c;
		var sh = _n0.d;
		return A2(
			elm$svg$Svg$g,
			evtlist,
			_List_fromArray(
				[
					A2(
					elm$svg$Svg$rect,
					_List_fromArray(
						[
							elm$svg$Svg$Attributes$x(model.y._),
							elm$svg$Svg$Attributes$y(model.y.aa),
							elm$svg$Svg$Attributes$width(model.y.W),
							elm$svg$Svg$Attributes$height(model.y.L),
							elm$svg$Svg$Attributes$rx('2'),
							elm$svg$Svg$Attributes$ry('2'),
							elm$svg$Svg$Attributes$style(
							'fill: #' + (theme.aS(0) + ';'))
						]),
					_List_Nil),
					A2(
					elm$svg$Svg$rect,
					_List_fromArray(
						[
							elm$svg$Svg$Attributes$x(sx),
							elm$svg$Svg$Attributes$y(sy),
							elm$svg$Svg$Attributes$width(sw),
							elm$svg$Svg$Attributes$height(sh),
							elm$svg$Svg$Attributes$rx('2'),
							elm$svg$Svg$Attributes$ry('2'),
							elm$svg$Svg$Attributes$style(
							'fill: #' + (theme.aS(
								model.n ? 3 : 4) + ';'))
						]),
					_List_Nil),
					A2(
					elm$virtual_dom$VirtualDom$map,
					function (_n2) {
						return author$project$SvgSlider$NoOp;
					},
					A2(elm$svg$Svg$g, _List_Nil, model.J))
				]));
	});
var author$project$SvgXY$NoOp = {$: 2};
var author$project$SvgXY$SvgPress = function (a) {
	return {$: 0, a: a};
};
var author$project$SvgXY$sliderEvt = F2(
	function (evtname, mkmsg) {
		return A2(
			elm$virtual_dom$VirtualDom$on,
			evtname,
			elm$virtual_dom$VirtualDom$Custom(
				A2(
					elm$json$Json$Decode$map,
					function (v) {
						return {
							a1: mkmsg(v),
							a5: true,
							a9: true
						};
					},
					elm$json$Json$Decode$value)));
	});
var author$project$SvgXY$onMouseDown = A2(author$project$SvgXY$sliderEvt, 'mousedown', author$project$SvgXY$SvgPress);
var author$project$SvgXY$SvgUnpress = function (a) {
	return {$: 1, a: a};
};
var author$project$SvgXY$onMouseLeave = A2(author$project$SvgXY$sliderEvt, 'mouseleave', author$project$SvgXY$SvgUnpress);
var author$project$SvgXY$SvgMoved = function (a) {
	return {$: 4, a: a};
};
var author$project$SvgXY$onMouseMove = A2(author$project$SvgXY$sliderEvt, 'mousemove', author$project$SvgXY$SvgMoved);
var author$project$SvgXY$onMouseUp = A2(author$project$SvgXY$sliderEvt, 'mouseup', author$project$SvgXY$SvgUnpress);
var author$project$SvgXY$SvgTouch = function (a) {
	return {$: 5, a: a};
};
var author$project$SvgXY$onTouchCancel = A2(
	author$project$SvgXY$sliderEvt,
	'touchcancel',
	function (e) {
		return author$project$SvgXY$SvgTouch(
			author$project$SvgTouch$SvgTouchCancel(e));
	});
var author$project$SvgXY$onTouchEnd = A2(
	author$project$SvgXY$sliderEvt,
	'touchend',
	function (e) {
		return author$project$SvgXY$SvgTouch(
			author$project$SvgTouch$SvgTouchEnd(e));
	});
var author$project$SvgXY$onTouchLeave = A2(
	author$project$SvgXY$sliderEvt,
	'touchleave',
	function (e) {
		return author$project$SvgXY$SvgTouch(
			author$project$SvgTouch$SvgTouchLeave(e));
	});
var author$project$SvgXY$onTouchMove = A2(
	author$project$SvgXY$sliderEvt,
	'touchmove',
	function (e) {
		return author$project$SvgXY$SvgTouch(
			author$project$SvgTouch$SvgTouchMove(e));
	});
var author$project$SvgXY$onTouchStart = A2(
	author$project$SvgXY$sliderEvt,
	'touchstart',
	function (e) {
		return author$project$SvgXY$SvgTouch(
			author$project$SvgTouch$SvgTouchStart(e));
	});
var author$project$SvgXY$buildEvtHandlerList = function (touchonly) {
	var te = _List_fromArray(
		[author$project$SvgXY$onTouchStart, author$project$SvgXY$onTouchEnd, author$project$SvgXY$onTouchCancel, author$project$SvgXY$onTouchLeave, author$project$SvgXY$onTouchMove]);
	var me = _List_fromArray(
		[author$project$SvgXY$onMouseDown, author$project$SvgXY$onMouseUp, author$project$SvgXY$onMouseLeave, author$project$SvgXY$onMouseMove]);
	return touchonly ? te : A2(elm$core$List$append, me, te);
};
var author$project$SvgXY$view = F2(
	function (theme, model) {
		var size = 10;
		var hsize = elm$core$Basics$round(size * 0.5);
		var sx = elm$core$String$fromInt(
			(elm$core$Basics$round(model.N.a * model.ay.W) + model.ay._) - hsize);
		var sy = elm$core$String$fromInt(
			(elm$core$Basics$round(model.N.b * model.ay.L) + model.ay.aa) - hsize);
		var evtlist = author$project$SvgXY$buildEvtHandlerList(model.aH);
		return A2(
			elm$svg$Svg$g,
			evtlist,
			_List_fromArray(
				[
					A2(
					elm$svg$Svg$rect,
					_List_fromArray(
						[
							elm$svg$Svg$Attributes$x(model.y._),
							elm$svg$Svg$Attributes$y(model.y.aa),
							elm$svg$Svg$Attributes$width(model.y.W),
							elm$svg$Svg$Attributes$height(model.y.L),
							elm$svg$Svg$Attributes$rx('2'),
							elm$svg$Svg$Attributes$ry('2'),
							elm$svg$Svg$Attributes$style(
							'fill: #' + (theme.aS(0) + ';'))
						]),
					_List_Nil),
					A2(
					elm$virtual_dom$VirtualDom$map,
					function (_n0) {
						return author$project$SvgXY$NoOp;
					},
					A2(elm$svg$Svg$g, _List_Nil, model.J)),
					A2(
					elm$svg$Svg$rect,
					_List_fromArray(
						[
							elm$svg$Svg$Attributes$x(sx),
							elm$svg$Svg$Attributes$y(sy),
							elm$svg$Svg$Attributes$width('10'),
							elm$svg$Svg$Attributes$height('10'),
							elm$svg$Svg$Attributes$rx('2'),
							elm$svg$Svg$Attributes$ry('2'),
							elm$svg$Svg$Attributes$style(
							'fill: #' + (theme.aS(
								model.n ? 3 : 4) + ';'))
						]),
					_List_Nil)
				]));
	});
var author$project$SvgControl$szview = F2(
	function (model, theme) {
		var controllst = elm$core$Dict$toList(model.d);
		return A2(
			elm$svg$Svg$g,
			_List_Nil,
			A2(
				elm$core$List$map,
				author$project$SvgControl$viewSvgControls(theme),
				controllst));
	});
var author$project$SvgControl$view = F2(
	function (theme, model) {
		switch (model.$) {
			case 0:
				var m = model.a;
				return A2(
					elm$virtual_dom$VirtualDom$map,
					author$project$SvgControl$CaButton,
					A2(author$project$SvgButton$view, theme, m));
			case 1:
				var m = model.a;
				return A2(
					elm$virtual_dom$VirtualDom$map,
					author$project$SvgControl$CaSlider,
					A2(author$project$SvgSlider$view, theme, m));
			case 2:
				var m = model.a;
				return A2(
					elm$virtual_dom$VirtualDom$map,
					author$project$SvgControl$CaXY,
					A2(author$project$SvgXY$view, theme, m));
			case 3:
				var m = model.a;
				return A2(
					elm$virtual_dom$VirtualDom$map,
					author$project$SvgControl$CaLabel,
					A2(author$project$SvgLabel$view, theme, m));
			default:
				var m = model.a;
				return A2(
					elm$virtual_dom$VirtualDom$map,
					author$project$SvgControl$CaSizer,
					A2(author$project$SvgControl$szview, m, theme));
		}
	});
var author$project$SvgControl$viewSvgControls = F2(
	function (theme, _n0) {
		var id = _n0.a;
		var model = _n0.b;
		return A2(
			elm$virtual_dom$VirtualDom$map,
			author$project$SvgControl$SzCMsg(id),
			A2(author$project$SvgControl$view, theme, model));
	});
var author$project$SvgControlPage$viewSvgControl = F2(
	function (theme, conmodel) {
		return A2(author$project$SvgControl$view, theme, conmodel);
	});
var elm$html$Html$div = _VirtualDom_node('div');
var elm$virtual_dom$VirtualDom$style = _VirtualDom_style;
var elm$html$Html$Attributes$style = elm$virtual_dom$VirtualDom$style;
var elm$svg$Svg$svg = elm$svg$Svg$trustedNode('svg');
var elm$svg$Svg$Attributes$viewBox = _VirtualDom_attribute('viewBox');
var author$project$SvgControlPage$view = function (model) {
	return A2(
		elm$html$Html$div,
		_List_fromArray(
			[
				A2(elm$html$Html$Attributes$style, 'margin', '0'),
				A2(elm$html$Html$Attributes$style, 'touch-action', 'none')
			]),
		_List_fromArray(
			[
				A2(
				elm$svg$Svg$svg,
				_List_fromArray(
					[
						elm$svg$Svg$Attributes$width(model.y.W),
						elm$svg$Svg$Attributes$height(model.y.L),
						elm$svg$Svg$Attributes$viewBox(model.y._ + (' ' + (model.y.aa + (' ' + (model.y.W + (' ' + model.y.L))))))
					]),
				_List_fromArray(
					[
						A2(
						elm$virtual_dom$VirtualDom$map,
						author$project$SvgControlPage$CMsg,
						A2(author$project$SvgControlPage$viewSvgControl, model.U, model.l))
					]))
			]));
};
var author$project$SvgTextSize$TextSizeReply = F2(
	function (width, controlId) {
		return {ae: controlId, X: width};
	});
var author$project$Util$andMap = F2(
	function (dca, dcab) {
		return A3(
			elm$json$Json$Decode$map2,
			F2(
				function (l, r) {
					return l(r);
				}),
			dcab,
			dca);
	});
var author$project$SvgTextSize$decodeTextSizeReply = A2(
	author$project$Util$andMap,
	A2(elm$json$Json$Decode$field, 'controlId', author$project$SvgThings$decodeControlId),
	A2(
		author$project$Util$andMap,
		A2(elm$json$Json$Decode$field, 'width', elm$json$Json$Decode$float),
		elm$json$Json$Decode$succeed(author$project$SvgTextSize$TextSizeReply)));
var author$project$WebSocket$Connect = function (a) {
	return {$: 0, a: a};
};
var elm$browser$Browser$Document = F2(
	function (title, body) {
		return {aQ: body, K: title};
	});
var elm$browser$Browser$External = function (a) {
	return {$: 1, a: a};
};
var elm$browser$Browser$Internal = function (a) {
	return {$: 0, a: a};
};
var elm$browser$Browser$Dom$NotFound = elm$core$Basics$identity;
var elm$core$Basics$never = function (_n0) {
	never:
	while (true) {
		var nvr = _n0;
		var $temp$_n0 = nvr;
		_n0 = $temp$_n0;
		continue never;
	}
};
var elm$core$Task$Perform = elm$core$Basics$identity;
var elm$core$Task$succeed = _Scheduler_succeed;
var elm$core$Task$init = elm$core$Task$succeed(0);
var elm$core$Task$andThen = _Scheduler_andThen;
var elm$core$Task$map = F2(
	function (func, taskA) {
		return A2(
			elm$core$Task$andThen,
			function (a) {
				return elm$core$Task$succeed(
					func(a));
			},
			taskA);
	});
var elm$core$Task$map2 = F3(
	function (func, taskA, taskB) {
		return A2(
			elm$core$Task$andThen,
			function (a) {
				return A2(
					elm$core$Task$andThen,
					function (b) {
						return elm$core$Task$succeed(
							A2(func, a, b));
					},
					taskB);
			},
			taskA);
	});
var elm$core$Task$sequence = function (tasks) {
	return A3(
		elm$core$List$foldr,
		elm$core$Task$map2(elm$core$List$cons),
		elm$core$Task$succeed(_List_Nil),
		tasks);
};
var elm$core$Platform$sendToApp = _Platform_sendToApp;
var elm$core$Task$spawnCmd = F2(
	function (router, _n0) {
		var task = _n0;
		return _Scheduler_spawn(
			A2(
				elm$core$Task$andThen,
				elm$core$Platform$sendToApp(router),
				task));
	});
var elm$core$Task$onEffects = F3(
	function (router, commands, state) {
		return A2(
			elm$core$Task$map,
			function (_n0) {
				return 0;
			},
			elm$core$Task$sequence(
				A2(
					elm$core$List$map,
					elm$core$Task$spawnCmd(router),
					commands)));
	});
var elm$core$Task$onSelfMsg = F3(
	function (_n0, _n1, _n2) {
		return elm$core$Task$succeed(0);
	});
var elm$core$Task$cmdMap = F2(
	function (tagger, _n0) {
		var task = _n0;
		return A2(elm$core$Task$map, tagger, task);
	});
_Platform_effectManagers['Task'] = _Platform_createManager(elm$core$Task$init, elm$core$Task$onEffects, elm$core$Task$onSelfMsg, elm$core$Task$cmdMap);
var elm$core$Task$command = _Platform_leaf('Task');
var elm$core$Task$perform = F2(
	function (toMessage, task) {
		return elm$core$Task$command(
			A2(elm$core$Task$map, toMessage, task));
	});
var elm$core$String$length = _String_length;
var elm$core$String$slice = _String_slice;
var elm$core$String$dropLeft = F2(
	function (n, string) {
		return (n < 1) ? string : A3(
			elm$core$String$slice,
			n,
			elm$core$String$length(string),
			string);
	});
var elm$core$String$startsWith = _String_startsWith;
var elm$url$Url$Http = 0;
var elm$url$Url$Https = 1;
var elm$core$String$indexes = _String_indexes;
var elm$core$String$isEmpty = function (string) {
	return string === '';
};
var elm$core$String$left = F2(
	function (n, string) {
		return (n < 1) ? '' : A3(elm$core$String$slice, 0, n, string);
	});
var elm$core$String$contains = _String_contains;
var elm$core$String$toInt = _String_toInt;
var elm$url$Url$Url = F6(
	function (protocol, host, port_, path, query, fragment) {
		return {aj: fragment, ak: host, ar: path, at: port_, a6: protocol, ax: query};
	});
var elm$url$Url$chompBeforePath = F5(
	function (protocol, path, params, frag, str) {
		if (elm$core$String$isEmpty(str) || A2(elm$core$String$contains, '@', str)) {
			return elm$core$Maybe$Nothing;
		} else {
			var _n0 = A2(elm$core$String$indexes, ':', str);
			if (!_n0.b) {
				return elm$core$Maybe$Just(
					A6(elm$url$Url$Url, protocol, str, elm$core$Maybe$Nothing, path, params, frag));
			} else {
				if (!_n0.b.b) {
					var i = _n0.a;
					var _n1 = elm$core$String$toInt(
						A2(elm$core$String$dropLeft, i + 1, str));
					if (_n1.$ === 1) {
						return elm$core$Maybe$Nothing;
					} else {
						var port_ = _n1;
						return elm$core$Maybe$Just(
							A6(
								elm$url$Url$Url,
								protocol,
								A2(elm$core$String$left, i, str),
								port_,
								path,
								params,
								frag));
					}
				} else {
					return elm$core$Maybe$Nothing;
				}
			}
		}
	});
var elm$url$Url$chompBeforeQuery = F4(
	function (protocol, params, frag, str) {
		if (elm$core$String$isEmpty(str)) {
			return elm$core$Maybe$Nothing;
		} else {
			var _n0 = A2(elm$core$String$indexes, '/', str);
			if (!_n0.b) {
				return A5(elm$url$Url$chompBeforePath, protocol, '/', params, frag, str);
			} else {
				var i = _n0.a;
				return A5(
					elm$url$Url$chompBeforePath,
					protocol,
					A2(elm$core$String$dropLeft, i, str),
					params,
					frag,
					A2(elm$core$String$left, i, str));
			}
		}
	});
var elm$url$Url$chompBeforeFragment = F3(
	function (protocol, frag, str) {
		if (elm$core$String$isEmpty(str)) {
			return elm$core$Maybe$Nothing;
		} else {
			var _n0 = A2(elm$core$String$indexes, '?', str);
			if (!_n0.b) {
				return A4(elm$url$Url$chompBeforeQuery, protocol, elm$core$Maybe$Nothing, frag, str);
			} else {
				var i = _n0.a;
				return A4(
					elm$url$Url$chompBeforeQuery,
					protocol,
					elm$core$Maybe$Just(
						A2(elm$core$String$dropLeft, i + 1, str)),
					frag,
					A2(elm$core$String$left, i, str));
			}
		}
	});
var elm$url$Url$chompAfterProtocol = F2(
	function (protocol, str) {
		if (elm$core$String$isEmpty(str)) {
			return elm$core$Maybe$Nothing;
		} else {
			var _n0 = A2(elm$core$String$indexes, '#', str);
			if (!_n0.b) {
				return A3(elm$url$Url$chompBeforeFragment, protocol, elm$core$Maybe$Nothing, str);
			} else {
				var i = _n0.a;
				return A3(
					elm$url$Url$chompBeforeFragment,
					protocol,
					elm$core$Maybe$Just(
						A2(elm$core$String$dropLeft, i + 1, str)),
					A2(elm$core$String$left, i, str));
			}
		}
	});
var elm$url$Url$fromString = function (str) {
	return A2(elm$core$String$startsWith, 'http://', str) ? A2(
		elm$url$Url$chompAfterProtocol,
		0,
		A2(elm$core$String$dropLeft, 7, str)) : (A2(elm$core$String$startsWith, 'https://', str) ? A2(
		elm$url$Url$chompAfterProtocol,
		1,
		A2(elm$core$String$dropLeft, 8, str)) : elm$core$Maybe$Nothing);
};
var elm$browser$Browser$document = _Browser_document;
var elm$browser$Browser$Events$Window = 1;
var elm$browser$Browser$Events$MySub = F3(
	function (a, b, c) {
		return {$: 0, a: a, b: b, c: c};
	});
var elm$browser$Browser$Events$State = F2(
	function (subs, pids) {
		return {as: pids, aE: subs};
	});
var elm$browser$Browser$Events$init = elm$core$Task$succeed(
	A2(elm$browser$Browser$Events$State, _List_Nil, elm$core$Dict$empty));
var elm$browser$Browser$Events$nodeToKey = function (node) {
	if (!node) {
		return 'd_';
	} else {
		return 'w_';
	}
};
var elm$browser$Browser$Events$addKey = function (sub) {
	var node = sub.a;
	var name = sub.b;
	return _Utils_Tuple2(
		_Utils_ap(
			elm$browser$Browser$Events$nodeToKey(node),
			name),
		sub);
};
var elm$browser$Browser$Events$Event = F2(
	function (key, event) {
		return {ai: event, am: key};
	});
var elm$core$Platform$sendToSelf = _Platform_sendToSelf;
var elm$browser$Browser$Events$spawn = F3(
	function (router, key, _n0) {
		var node = _n0.a;
		var name = _n0.b;
		var actualNode = function () {
			if (!node) {
				return _Browser_doc;
			} else {
				return _Browser_window;
			}
		}();
		return A2(
			elm$core$Task$map,
			function (value) {
				return _Utils_Tuple2(key, value);
			},
			A3(
				_Browser_on,
				actualNode,
				name,
				function (event) {
					return A2(
						elm$core$Platform$sendToSelf,
						router,
						A2(elm$browser$Browser$Events$Event, key, event));
				}));
	});
var elm$core$Dict$foldl = F3(
	function (func, acc, dict) {
		foldl:
		while (true) {
			if (dict.$ === -2) {
				return acc;
			} else {
				var key = dict.b;
				var value = dict.c;
				var left = dict.d;
				var right = dict.e;
				var $temp$func = func,
					$temp$acc = A3(
					func,
					key,
					value,
					A3(elm$core$Dict$foldl, func, acc, left)),
					$temp$dict = right;
				func = $temp$func;
				acc = $temp$acc;
				dict = $temp$dict;
				continue foldl;
			}
		}
	});
var elm$core$Dict$merge = F6(
	function (leftStep, bothStep, rightStep, leftDict, rightDict, initialResult) {
		var stepState = F3(
			function (rKey, rValue, _n0) {
				stepState:
				while (true) {
					var list = _n0.a;
					var result = _n0.b;
					if (!list.b) {
						return _Utils_Tuple2(
							list,
							A3(rightStep, rKey, rValue, result));
					} else {
						var _n2 = list.a;
						var lKey = _n2.a;
						var lValue = _n2.b;
						var rest = list.b;
						if (_Utils_cmp(lKey, rKey) < 0) {
							var $temp$rKey = rKey,
								$temp$rValue = rValue,
								$temp$_n0 = _Utils_Tuple2(
								rest,
								A3(leftStep, lKey, lValue, result));
							rKey = $temp$rKey;
							rValue = $temp$rValue;
							_n0 = $temp$_n0;
							continue stepState;
						} else {
							if (_Utils_cmp(lKey, rKey) > 0) {
								return _Utils_Tuple2(
									list,
									A3(rightStep, rKey, rValue, result));
							} else {
								return _Utils_Tuple2(
									rest,
									A4(bothStep, lKey, lValue, rValue, result));
							}
						}
					}
				}
			});
		var _n3 = A3(
			elm$core$Dict$foldl,
			stepState,
			_Utils_Tuple2(
				elm$core$Dict$toList(leftDict),
				initialResult),
			rightDict);
		var leftovers = _n3.a;
		var intermediateResult = _n3.b;
		return A3(
			elm$core$List$foldl,
			F2(
				function (_n4, result) {
					var k = _n4.a;
					var v = _n4.b;
					return A3(leftStep, k, v, result);
				}),
			intermediateResult,
			leftovers);
	});
var elm$core$Dict$union = F2(
	function (t1, t2) {
		return A3(elm$core$Dict$foldl, elm$core$Dict$insert, t2, t1);
	});
var elm$core$Process$kill = _Scheduler_kill;
var elm$browser$Browser$Events$onEffects = F3(
	function (router, subs, state) {
		var stepRight = F3(
			function (key, sub, _n6) {
				var deads = _n6.a;
				var lives = _n6.b;
				var news = _n6.c;
				return _Utils_Tuple3(
					deads,
					lives,
					A2(
						elm$core$List$cons,
						A3(elm$browser$Browser$Events$spawn, router, key, sub),
						news));
			});
		var stepLeft = F3(
			function (_n4, pid, _n5) {
				var deads = _n5.a;
				var lives = _n5.b;
				var news = _n5.c;
				return _Utils_Tuple3(
					A2(elm$core$List$cons, pid, deads),
					lives,
					news);
			});
		var stepBoth = F4(
			function (key, pid, _n2, _n3) {
				var deads = _n3.a;
				var lives = _n3.b;
				var news = _n3.c;
				return _Utils_Tuple3(
					deads,
					A3(elm$core$Dict$insert, key, pid, lives),
					news);
			});
		var newSubs = A2(elm$core$List$map, elm$browser$Browser$Events$addKey, subs);
		var _n0 = A6(
			elm$core$Dict$merge,
			stepLeft,
			stepBoth,
			stepRight,
			state.as,
			elm$core$Dict$fromList(newSubs),
			_Utils_Tuple3(_List_Nil, elm$core$Dict$empty, _List_Nil));
		var deadPids = _n0.a;
		var livePids = _n0.b;
		var makeNewPids = _n0.c;
		return A2(
			elm$core$Task$andThen,
			function (pids) {
				return elm$core$Task$succeed(
					A2(
						elm$browser$Browser$Events$State,
						newSubs,
						A2(
							elm$core$Dict$union,
							livePids,
							elm$core$Dict$fromList(pids))));
			},
			A2(
				elm$core$Task$andThen,
				function (_n1) {
					return elm$core$Task$sequence(makeNewPids);
				},
				elm$core$Task$sequence(
					A2(elm$core$List$map, elm$core$Process$kill, deadPids))));
	});
var elm$core$List$maybeCons = F3(
	function (f, mx, xs) {
		var _n0 = f(mx);
		if (!_n0.$) {
			var x = _n0.a;
			return A2(elm$core$List$cons, x, xs);
		} else {
			return xs;
		}
	});
var elm$core$List$filterMap = F2(
	function (f, xs) {
		return A3(
			elm$core$List$foldr,
			elm$core$List$maybeCons(f),
			_List_Nil,
			xs);
	});
var elm$browser$Browser$Events$onSelfMsg = F3(
	function (router, _n0, state) {
		var key = _n0.am;
		var event = _n0.ai;
		var toMessage = function (_n2) {
			var subKey = _n2.a;
			var _n3 = _n2.b;
			var node = _n3.a;
			var name = _n3.b;
			var decoder = _n3.c;
			return _Utils_eq(subKey, key) ? A2(_Browser_decodeEvent, decoder, event) : elm$core$Maybe$Nothing;
		};
		var messages = A2(elm$core$List$filterMap, toMessage, state.aE);
		return A2(
			elm$core$Task$andThen,
			function (_n1) {
				return elm$core$Task$succeed(state);
			},
			elm$core$Task$sequence(
				A2(
					elm$core$List$map,
					elm$core$Platform$sendToApp(router),
					messages)));
	});
var elm$browser$Browser$Events$subMap = F2(
	function (func, _n0) {
		var node = _n0.a;
		var name = _n0.b;
		var decoder = _n0.c;
		return A3(
			elm$browser$Browser$Events$MySub,
			node,
			name,
			A2(elm$json$Json$Decode$map, func, decoder));
	});
_Platform_effectManagers['Browser.Events'] = _Platform_createManager(elm$browser$Browser$Events$init, elm$browser$Browser$Events$onEffects, elm$browser$Browser$Events$onSelfMsg, 0, elm$browser$Browser$Events$subMap);
var elm$browser$Browser$Events$subscription = _Platform_leaf('Browser.Events');
var elm$browser$Browser$Events$on = F3(
	function (node, name, decoder) {
		return elm$browser$Browser$Events$subscription(
			A3(elm$browser$Browser$Events$MySub, node, name, decoder));
	});
var elm$browser$Browser$Events$onResize = function (func) {
	return A3(
		elm$browser$Browser$Events$on,
		1,
		'resize',
		A2(
			elm$json$Json$Decode$field,
			'target',
			A3(
				elm$json$Json$Decode$map2,
				func,
				A2(elm$json$Json$Decode$field, 'innerWidth', elm$json$Json$Decode$int),
				A2(elm$json$Json$Decode$field, 'innerHeight', elm$json$Json$Decode$int))));
};
var elm$core$Basics$composeL = F3(
	function (g, f, x) {
		return g(
			f(x));
	});
var elm$core$Platform$Sub$batch = _Platform_batch;
var elm$core$Platform$Sub$map = _Platform_map;
var elm$html$Html$map = elm$virtual_dom$VirtualDom$map;
var author$project$Main$main = elm$browser$Browser$document(
	{
		a0: function (flags) {
			var _n0 = author$project$Main$init(flags);
			var mod = _n0.a;
			var cmd = _n0.b;
			return _Utils_Tuple2(
				mod,
				elm$core$Platform$Cmd$batch(
					_List_fromArray(
						[
							author$project$Main$wssend(
							author$project$WebSocket$Connect(
								{aO: mod.Y, ap: 'touchpage', a6: 'rust-websocket'})),
							author$project$Main$commandToCmd(cmd)
						])));
		},
		bb: function (_n1) {
			return elm$core$Platform$Sub$batch(
				_List_fromArray(
					[
						A2(
						elm$core$Platform$Sub$map,
						author$project$Main$ScpMsg,
						elm$browser$Browser$Events$onResize(
							F2(
								function (a, b) {
									return author$project$SvgControlPage$Resize(
										A2(author$project$Util$RectSize, a, b));
								}))),
						author$project$Main$wsreceive,
						author$project$Main$receiveTextMetrics(
						A2(
							elm$core$Basics$composeL,
							author$project$Main$TextSize,
							elm$json$Json$Decode$decodeValue(author$project$SvgTextSize$decodeTextSizeReply)))
					]));
		},
		bc: F2(
			function (msg, mod) {
				switch (msg.$) {
					case 2:
						var sm = msg.a;
						var _n3 = A2(author$project$SvgControlPage$update, sm, mod.i);
						var umod = _n3.a;
						var cmd = _n3.b;
						return _Utils_Tuple2(
							_Utils_update(
								mod,
								{i: umod}),
							author$project$Main$commandToCmd(cmd));
					case 0:
						var x = msg.a;
						if (!x.$) {
							if (x.a.$ === 1) {
								var wsd = x.a.a;
								var _n5 = A2(
									author$project$SvgControlPage$update,
									author$project$SvgControlPage$JsonMsg(wsd.aU),
									mod.i);
								var scpModel = _n5.a;
								var scpCommand = _n5.b;
								return _Utils_Tuple2(
									_Utils_update(
										mod,
										{i: scpModel}),
									author$project$Main$commandToCmd(scpCommand));
							} else {
								var wse = x.a.a;
								return _Utils_Tuple2(mod, elm$core$Platform$Cmd$none);
							}
						} else {
							return _Utils_Tuple2(mod, elm$core$Platform$Cmd$none);
						}
					default:
						var ts = msg.a;
						if (!ts.$) {
							var tsr = ts.a;
							return _Utils_Tuple2(
								_Utils_update(
									mod,
									{
										i: A2(author$project$SvgControlPage$onTextSize, tsr, mod.i)
									}),
								elm$core$Platform$Cmd$none);
						} else {
							return _Utils_Tuple2(mod, elm$core$Platform$Cmd$none);
						}
				}
			}),
		be: function (model) {
			return A2(
				elm$browser$Browser$Document,
				'svg control',
				_List_fromArray(
					[
						A2(
						elm$html$Html$map,
						author$project$Main$ScpMsg,
						author$project$SvgControlPage$view(model.i))
					]));
		}
	});
_Platform_export({'Main':{'init':author$project$Main$main(
	A2(
		elm$json$Json$Decode$andThen,
		function (wsport) {
			return A2(
				elm$json$Json$Decode$andThen,
				function (width) {
					return A2(
						elm$json$Json$Decode$andThen,
						function (location) {
							return A2(
								elm$json$Json$Decode$andThen,
								function (height) {
									return elm$json$Json$Decode$succeed(
										{M: height, N: location, X: width, Z: wsport});
								},
								A2(elm$json$Json$Decode$field, 'height', elm$json$Json$Decode$int));
						},
						A2(elm$json$Json$Decode$field, 'location', elm$json$Json$Decode$string));
				},
				A2(elm$json$Json$Decode$field, 'width', elm$json$Json$Decode$int));
		},
		A2(elm$json$Json$Decode$field, 'wsport', elm$json$Json$Decode$int)))(0)}});}(this));</script>
<script>
  
  function getTextWidth (t, f) {
    var blah = getTextMetrics(t, f);
    return blah.width; 
  };

  function getTextMetrics (text, font) {
     // re-use canvas object for better performance
     var canvas = getTextMetrics.canvas || (getTextMetrics.canvas = document.createElement("canvas"));
     var context = canvas.getContext("2d");
     context.font = font;
     var metrics = context.measureText(text);
     return metrics;
    };

  function requestTextSize (rts) {
    // console.log (" rts: " + JSON.stringify(rts, null, 4)); 
    metrics = getTextMetrics(rts.string, rts.font);
    // console.log( "getTextMetricssss: " + metrics.width);
    var reply = { width : metrics.width, controlId : rts.controlId };
    // console.log (" reply: " + JSON.stringify(reply, null, 4));
    app.ports.receiveTextMetrics.send(reply);
  }
 
</script>
<script>
  var mySockets = {};

  function sendSocketCommand(wat) {
    // console.log( "ssc: " +  JSON.stringify(wat, null, 4));
    if (wat.cmd == "connect") 
    {
      // console.log("connecting!");
      socket = new WebSocket(wat.address, wat.protocol);
      socket.onmessage = function (event) {
        // console.log( "onmessage: " +  JSON.stringify(event.data, null, 4));
        app.ports.receiveSocketMsg.send({ name : wat.name
                                        , msg : "data"
                                        , data : event.data} );
    	}
    	mySockets[wat.name] = socket;
    }
    else if (wat.cmd == "send")
    {
      // console.log("sending to socket: " + wat.name );
      mySockets[wat.name].send(wat.content); 
    }
    else if (wat.cmd == "close")
    {
      // console.log("closing socket: " + wat.name);
      mySockets[wat.name].close();
      delete mySockets[wat.name];
    }
  }

</script>
<script>
    var app = Elm.Main.init( 
        { flags: { location : document.location.origin || "", 
                   wsport : {{websockets-port}},
                   width : window.innerWidth, 
                   height : window.innerHeight
                 }, 
          node: document.getElementById("elm") 
        });
    if (document.getElementById("elm"))
    {
      document.getElementById("elm").innerText = 'This is a headless program, meaning there is nothing to show here.\n\nI started the program anyway though, and you can access it as `app` in the developer console.';
    }
    app.ports.sendSocketCommand.subscribe(sendSocketCommand);
    app.ports.requestTextSize.subscribe(requestTextSize);
</script>
</body>
</html>
"##;
