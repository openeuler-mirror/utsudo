#include <config.h>

#include <sys/types.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <ctype.h>
#include <errno.h>
#include <fcntl.h>
#include <time.h>
#include <netdb.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/uio.h>
#define DEFAULT_TEXT_DOMAIN	"sudo"
#include "sudo_gettext.h"	/* must be included before sudo_compat.h */

#include "sudo_compat.h"
#include "sudo_fatal.h"
#include "sudo_plugin.h"
#include "sudo_debug.h"
#include "sudo_conf.h"
#include "sudo_util.h"

#ifdef HAVE_STDBOOL_H
# include <stdbool.h>
#else
# include "compat/stdbool.h"
#endif /* HAVE_STDBOOL_H */
#include "sudo_queue.h"
#include "sudo_lbuf.h"


#ifndef HAVE_GETADDRINFO
# include "compat/getaddrinfo.h"
#endif


void warning(const char *errstr, const char *fmt, va_list ap);

/******************************  lbuf.c  **********************************/

bool sudo_lbuf_expand(struct sudo_lbuf *lbuf, int extra);

/*
 * Parse the format and append strings, only %s and %% escapes are supported.
 * Any characters in set are quoted with a backslash.
 */
bool
sudo_lbuf_append_quoted_v1(struct sudo_lbuf *lbuf, const char *set, const char *fmt, ...)
{
    int len, saved_len = lbuf->len;
    bool ret = false;
    char *cp, *s;
    va_list ap;
    debug_decl(sudo_lbuf_append_quoted, SUDO_DEBUG_UTIL)

    if (sudo_lbuf_error(lbuf))
	debug_return_bool(false);

    va_start(ap, fmt);
    while (*fmt != '\0') {
	if (fmt[0] == '%' && fmt[1] == 's') {
	    if ((s = va_arg(ap, char *)) == NULL)
		s = "(NULL)";
	    while ((cp = strpbrk(s, set)) != NULL) {
		len = (int)(cp - s);
		if (!sudo_lbuf_expand(lbuf, len + 2))
		    goto done;
		memcpy(lbuf->buf + lbuf->len, s, len);
		lbuf->len += len;
		lbuf->buf[lbuf->len++] = '\\';
		lbuf->buf[lbuf->len++] = *cp;
		s = cp + 1;
	    }
	    if (*s != '\0') {
		len = strlen(s);
		if (!sudo_lbuf_expand(lbuf, len))
		    goto done;
		memcpy(lbuf->buf + lbuf->len, s, len);
		lbuf->len += len;
	    }
	    fmt += 2;
	    continue;
	}
	if (!sudo_lbuf_expand(lbuf, 2))
	    goto done;
	if (strchr(set, *fmt) != NULL)
	    lbuf->buf[lbuf->len++] = '\\';
	lbuf->buf[lbuf->len++] = *fmt++;
    }
    ret = true;

done:
    if (!ret)
	lbuf->len = saved_len;
    if (lbuf->size != 0)
	lbuf->buf[lbuf->len] = '\0';
    va_end(ap);

    debug_return_bool(ret);
}

/*
 * Parse the format and append strings, only %s and %% escapes are supported.
 */
bool
sudo_lbuf_append_v1(struct sudo_lbuf *lbuf, const char *fmt, ...)
{
    int len, saved_len = lbuf->len;
    bool ret = false;
    va_list ap;
    char *s;
    debug_decl(sudo_lbuf_append, SUDO_DEBUG_UTIL)

    if (sudo_lbuf_error(lbuf))
	debug_return_bool(false);

    va_start(ap, fmt);
    while (*fmt != '\0') {
	if (fmt[0] == '%' && fmt[1] == 's') {
	    if ((s = va_arg(ap, char *)) == NULL)
		s = "(NULL)";
	    len = strlen(s);
	    if (!sudo_lbuf_expand(lbuf, len))
		goto done;
	    memcpy(lbuf->buf + lbuf->len, s, len);
	    lbuf->len += len;
	    fmt += 2;
	    continue;
	}
	if (!sudo_lbuf_expand(lbuf, 1))
	    goto done;
	lbuf->buf[lbuf->len++] = *fmt++;
    }
    ret = true;

done:
    if (!ret)
	lbuf->len = saved_len;
    if (lbuf->size != 0)
	lbuf->buf[lbuf->len] = '\0';
    va_end(ap);

    debug_return_bool(ret);
}

/* XXX - check output function return value */
static void
sudo_lbuf_println(struct sudo_lbuf *lbuf, char *line, int len)
{
    char *cp, save;
    int i, have, contlen = 0;
    int indent = lbuf->indent;
    bool is_comment = false;
    debug_decl(sudo_lbuf_println, SUDO_DEBUG_UTIL)

    /* Comment lines don't use continuation and only indent is for "# " */
    if (line[0] == '#' && isblank((unsigned char)line[1])) {
	is_comment = true;
	indent = 2;
    }
    if (lbuf->continuation != NULL && !is_comment)
	contlen = strlen(lbuf->continuation);

    /*
     * Print the buffer, splitting the line as needed on a word
     * boundary.
     */
    cp = line;
    have = lbuf->cols;
    while (cp != NULL && *cp != '\0') {
	char *ep = NULL;
	int need = len - (int)(cp - line);

	if (need > have) {
	    have -= contlen;		/* subtract for continuation char */
	    if ((ep = memrchr(cp, ' ', have)) == NULL)
		ep = memchr(cp + have, ' ', need - have);
	    if (ep != NULL)
		need = (int)(ep - cp);
	}
	if (cp != line) {
	    if (is_comment) {
		lbuf->output("# ");
	    } else {
		/* indent continued lines */
		/* XXX - build up string instead? */
		for (i = 0; i < indent; i++)
		    lbuf->output(" ");
	    }
	}
	/* NUL-terminate cp for the output function and restore afterwards */
	save = cp[need];
	cp[need] = '\0';
	lbuf->output(cp);
	cp[need] = save;
	cp = ep;

	/*
	 * If there is more to print, reset have, incremement cp past
	 * the whitespace, and print a line continuaton char if needed.
	 */
	if (cp != NULL) {
	    have = lbuf->cols - indent;
	    ep = line + len;
	    while (cp < ep && isblank((unsigned char)*cp)) {
		cp++;
	    }
	    if (contlen)
		lbuf->output(lbuf->continuation);
	}
	lbuf->output("\n");
    }

    debug_return;
}

bool
sudo_lbuf_expand(struct sudo_lbuf *lbuf, int extra)
{
    debug_decl(sudo_lbuf_expand, SUDO_DEBUG_UTIL)

    if (lbuf->len + extra + 1 >= lbuf->size) {
	char *new_buf;
	int new_size = lbuf->size;

	do {
	    new_size += 256;
	} while (lbuf->len + extra + 1 >= new_size);
	if ((new_buf = realloc(lbuf->buf, new_size)) == NULL) {
	    sudo_debug_printf(SUDO_DEBUG_ERROR|SUDO_DEBUG_LINENO,
		"unable to allocate memory");
	    lbuf->error = 1;
	    debug_return_bool(false);
	}
	lbuf->buf = new_buf;
	lbuf->size = new_size;
    }
    debug_return_bool(true);
}



/**************************************************************************/
/******************************  fata.c  **********************************/

static bool (*sudo_warn_setlocale)(bool, int *);
static bool (*sudo_warn_setlocale_prev)(bool, int *);

struct sudo_fatal_callback {
    SLIST_ENTRY(sudo_fatal_callback) entries;
    void (*func)(void);
};
SLIST_HEAD(sudo_fatal_callback_list, sudo_fatal_callback);
static struct sudo_fatal_callback_list callbacks = SLIST_HEAD_INITIALIZER(&callbacks);

sudo_conv_t sudo_warn_conversation;

 void
do_cleanup(void)
{
    struct sudo_fatal_callback *cb;

    /* Run callbacks, removing them from the list as we go. */
    while ((cb = SLIST_FIRST(&callbacks)) != NULL) {
	SLIST_REMOVE_HEAD(&callbacks, entries);
	cb->func();
	free(cb);
    }
}

void
sudo_fatal_nodebug_v1(const char *fmt, ...)
{
    va_list ap;

    va_start(ap, fmt);
    warning(strerror(errno), fmt, ap);
    va_end(ap);
    do_cleanup();
    exit(EXIT_FAILURE);
}

void
sudo_fatalx_nodebug_v1(const char *fmt, ...)
{
    va_list ap;

    va_start(ap, fmt);
    warning(NULL, fmt, ap);
    va_end(ap);
    do_cleanup();
    exit(EXIT_FAILURE);
}

void
sudo_warn_nodebug_v1(const char *fmt, ...)
{
    va_list ap;

    va_start(ap, fmt);
    warning(strerror(errno), fmt, ap);
    va_end(ap);
}

void
sudo_warnx_nodebug_v1(const char *fmt, ...)
{
    va_list ap;
    va_start(ap, fmt);
    warning(NULL, fmt, ap);
    va_end(ap);
}

void
sudo_gai_fatal_nodebug_v1(int errnum, const char *fmt, ...)
{
    va_list ap;

    va_start(ap, fmt);
    warning(gai_strerror(errnum), fmt, ap);
    va_end(ap);
    do_cleanup();
    exit(EXIT_FAILURE);
}

void
sudo_gai_warn_nodebug_v1(int errnum, const char *fmt, ...)
{
    va_list ap;

    va_start(ap, fmt);
    warning(gai_strerror(errnum), fmt, ap);
    va_end(ap);
}

void
sudo_vfatal_nodebug_v1(const char *fmt, va_list ap)
{
    warning(strerror(errno), fmt, ap);
    do_cleanup();
    exit(EXIT_FAILURE);
}

void
sudo_vfatalx_nodebug_v1(const char *fmt, va_list ap)
{
    warning(NULL, fmt, ap);
    do_cleanup();
    exit(EXIT_FAILURE);
}

void
sudo_vwarn_nodebug_v1(const char *fmt, va_list ap)
{
    warning(strerror(errno), fmt, ap);
}

void
sudo_vwarnx_nodebug_v1(const char *fmt, va_list ap)
{
    warning(NULL, fmt, ap);
}

void
sudo_gai_vfatal_nodebug_v1(int errnum, const char *fmt, va_list ap)
{
    warning(gai_strerror(errnum), fmt, ap);
    do_cleanup();
    exit(EXIT_FAILURE);
}

void
sudo_gai_vwarn_nodebug_v1(int errnum, const char *fmt, va_list ap)
{
    warning(gai_strerror(errnum), fmt, ap);
}

void
warning(const char *errstr, const char *fmt, va_list ap)
{
    int cookie;

    /* Set user locale if setter was specified. */
    if (sudo_warn_setlocale != NULL)
	sudo_warn_setlocale(false, &cookie);

    if (sudo_warn_conversation != NULL) {
	struct sudo_conv_message msgs[6];
	char static_buf[1024], *buf = static_buf;
	int nmsgs = 0;

	/* Use conversation function. */
        msgs[nmsgs].msg_type = SUDO_CONV_ERROR_MSG;
	    msgs[nmsgs++].msg = getprogname();
        if (fmt != NULL) {
		va_list ap2;
		int buflen;

		/* Use static buffer if possible, else dynamic. */
		va_copy(ap2, ap);
		buflen = vsnprintf(static_buf, sizeof(static_buf), fmt, ap2);
		va_end(ap2);
		if (buflen >= ssizeof(static_buf)) {
		    buf = malloc(++buflen);
		    if (buf != NULL)
			(void)vsnprintf(buf, buflen, fmt, ap);
		    else
			buf = static_buf;
		}
		msgs[nmsgs].msg_type = SUDO_CONV_ERROR_MSG;
		msgs[nmsgs++].msg = ": ";
		msgs[nmsgs].msg_type = SUDO_CONV_ERROR_MSG;
		msgs[nmsgs++].msg = buf;
        }
        if (errstr != NULL) {
	    msgs[nmsgs].msg_type = SUDO_CONV_ERROR_MSG;
	    msgs[nmsgs++].msg = ": ";
	    msgs[nmsgs].msg_type = SUDO_CONV_ERROR_MSG;
	    msgs[nmsgs++].msg = errstr;
        }
	msgs[nmsgs].msg_type = SUDO_CONV_ERROR_MSG;
	msgs[nmsgs++].msg = "\n";
	sudo_warn_conversation(nmsgs, msgs, NULL, NULL);
	if (buf != static_buf)
	    free(buf);
    } else {
	/* Write to the standard error. */
        fputs(getprogname(), stderr);
        if (fmt != NULL) {
                fputs(": ", stderr);
                vfprintf(stderr, fmt, ap);
        }
        if (errstr != NULL) {
            fputs(": ", stderr);
            fputs(errstr, stderr);
        }
        putc('\n', stderr);
    }

    /* Restore old locale as needed. */
    if (sudo_warn_setlocale != NULL)
	sudo_warn_setlocale(true, &cookie);
}

/**************************************************************************/
/***************************  sudo_debug.c  *******************************/

/* Support up to 10 instances. */
#define SUDO_DEBUG_INSTANCE_MAX 10
static struct sudo_debug_instance *sudo_debug_instances[SUDO_DEBUG_INSTANCE_MAX];
static int sudo_debug_last_instance = -1;
SLIST_HEAD(sudo_debug_output_list, sudo_debug_output);

/* Default instance index to use for common utility functions. */
int sudo_debug_active_instance = -1;

struct sudo_debug_output {
    SLIST_ENTRY(sudo_debug_output) entries;
    char *filename;
    int *settings;
    int fd;
};

struct sudo_debug_instance {
    char *program;
    const char *const *subsystems;
    const unsigned int *subsystem_ids;
    unsigned int max_subsystem;
    struct sudo_debug_output_list outputs;
};

void
sudo_debug_vprintf2_v1(const char *func, const char *file, int lineno, int level,
    const char *fmt, va_list ap)
{
    int buflen, pri, saved_errno = errno;
    unsigned int subsys;
    char static_buf[1024], *buf = static_buf;
    struct sudo_debug_instance *instance;
    struct sudo_debug_output *output;
    debug_decl_func(sudo_debug_vprintf2);

    if (sudo_debug_active_instance == -1)
	goto out;

    /* Extract priority and subsystem from level. */
    pri = SUDO_DEBUG_PRI(level);
    subsys = SUDO_DEBUG_SUBSYS(level);

    /* Find matching instance. */
    if (sudo_debug_active_instance > sudo_debug_last_instance) {
	sudo_warnx_nodebug("%s: invalid instance ID %d, max %d",
	    __func__, sudo_debug_active_instance, sudo_debug_last_instance);
	goto out;
    }
    instance = sudo_debug_instances[sudo_debug_active_instance];
    if (instance == NULL) {
	sudo_warnx_nodebug("%s: unregistered instance index %d", __func__,
	    sudo_debug_active_instance);
	goto out;
    }

    SLIST_FOREACH(output, &instance->outputs, entries) {
	/* Make sure we want debug info at this level. */
	if (subsys <= instance->max_subsystem && output->settings[subsys] >= pri) {
	    va_list ap2;

	    /* Operate on a copy of ap to support multiple outputs. */
	    va_copy(ap2, ap);
	    buflen = fmt ? vsnprintf(static_buf, sizeof(static_buf), fmt, ap2) : 0;
	    va_end(ap2);
	    if (buflen >= ssizeof(static_buf)) {
		va_list ap3;

		/* Not enough room in static buf, allocate dynamically. */
		va_copy(ap3, ap);
		buflen = vasprintf(&buf, fmt, ap3);
		va_end(ap3);
	    }
	    if (buflen != -1) {
		int errcode = ISSET(level, SUDO_DEBUG_ERRNO) ? saved_errno : 0;
		if (ISSET(level, SUDO_DEBUG_LINENO))
		    sudo_debug_write2(output->fd, func, file, lineno, buf, buflen, errcode);
		else
		    sudo_debug_write2(output->fd, NULL, NULL, 0, buf, buflen, errcode);
		if (buf != static_buf) {
		    free(buf);
		    buf = static_buf;
		}
	    }
	}
    }
out:
    errno = saved_errno;
}


void
sudo_debug_printf2_v1(const char *func, const char *file, int lineno, int level,
    const char *fmt, ...)
{
    va_list ap;

    va_start(ap, fmt);
    sudo_debug_vprintf2(func, file, lineno, level, fmt, ap);
    va_end(ap);
}

/**************************************************************************/
