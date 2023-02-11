#!/bin/bash
set -x -e

# Remove Gee dependency, as we can't bind this yet
xmlstarlet ed -L \
	-d '///_:include[@name="Gee"]' \
	Granite-7.0.gir

# Remove functions with Gee return types
xmlstarlet ed -L \
	-d '///_:function/_:return-value/_:type[contains(@name, "Gee.")]/../..' \
	Granite-7.0.gir

# Rename c:identifier to c:type on constants
xmlstarlet ed -L \
	-r '///_:constant/@c:identifier' -v 'type' \
	Granite-7.0.gir

# Fix weirdly double namespaced stuff
xmlstarlet ed -L \
    -u '///_:type[starts-with(@name, "Granite.Granite")]/@name' \
    -x 'substring-after(., "Granite.Granite")' \
    Granite-7.0.gir

xmlstarlet ed -L \
    -u '///_:implements[starts-with(@name, "Granite.Granite")]/@name' \
    -x 'substring-after(., "Granite.Granite")' \
    Granite-7.0.gir

# Add shared library to namespace
xmlstarlet ed -L \
    -u '///_:namespace/@shared-library' -v 'libgranite-7.so.7' \
    -i '///_:namespace[not(@shared-library)]' -t attr -n 'shared-library' -v 'libgranite-7.so.7' \
    Granite-7.0.gir

# Remove const from constant string types
xmlstarlet ed -L \
    -u '///_:constant/_:type[@c:type="const gchar*"]/@c:type' -v 'gchar*' \
    Granite-7.0.gir

# Pass GdkRGBA by reference
xmlstarlet ed -L \
    -u '///_:type[@c:type="GdkRGBA"]/@c:type' -v 'GdkRGBA*' \
    Granite-7.0.gir