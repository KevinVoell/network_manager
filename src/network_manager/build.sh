#!/bin/bash
for filename in ../../NetworkManager/introspection/*.xml; do
	zbus-xmlgen file "$filename"
done
