# UW Groups CLI

Simple command line utility for working with the UW Groups Service. The tool is limited in scope, with a focus on common operations and automatibility.

* Adding users to groups
* Removing users from groups
* Fetching group membership list
* Fetching a user's group memberships

## Requirements

* libcurl
* a client-server certificate

## Building

During the build process, set the environment variable "GROUPS_API" to the service you wish to use. For example, the eval environment:

```
export GROUPS_API="https://eval.groups.uw.edu/group_sws/v3"
```

## Examples

### Adding a user to a group

Add a user to a single group:

```
groupstool add-member /path/to/my/cert u_my_group someuser
```

Adding a user to several groups:

```
for grp in u_group1 u_group2 u_group3; do groupstool add-member /path/to/my/cert ${grp} someuser; done
```

### Removing a user from a group

Remove a user from a single group:

```
groupstool remove-member /path/to/my/cert u_my_group someuser
```

Removing a user from several groups:

```
for grp in u_group1 u_group2 u_group3; do groupstool remove-member /path/to/my/cert ${grp} someuser; done
```

### Fetching a user's group memberships


Simply printing the user's group memberships:

```
groupstool groups-by-member /path/to/my/cert u_my_group someuser
```

Removing a user from a scoped set of group memberships:

```
for grp in $(groupstool groups-by-member /path/to/my/cert u_my_group someuser | grep u_group_stem); do groupstool remove-member /path/to/my/cert ${grp} someuser; done
```