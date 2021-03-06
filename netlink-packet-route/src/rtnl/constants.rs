pub use netlink_packet_core::constants::*;

pub const RTM_BASE: u16 = 16;
pub const RTM_NEWLINK: u16 = 16;
pub const RTM_DELLINK: u16 = 17;
pub const RTM_GETLINK: u16 = 18;
pub const RTM_SETLINK: u16 = 19;
pub const RTM_NEWADDR: u16 = 20;
pub const RTM_DELADDR: u16 = 21;
pub const RTM_GETADDR: u16 = 22;
pub const RTM_NEWROUTE: u16 = 24;
pub const RTM_DELROUTE: u16 = 25;
pub const RTM_GETROUTE: u16 = 26;
pub const RTM_NEWNEIGH: u16 = 28;
pub const RTM_DELNEIGH: u16 = 29;
pub const RTM_GETNEIGH: u16 = 30;
pub const RTM_NEWRULE: u16 = 32;
pub const RTM_DELRULE: u16 = 33;
pub const RTM_GETRULE: u16 = 34;
pub const RTM_NEWQDISC: u16 = 36;
pub const RTM_DELQDISC: u16 = 37;
pub const RTM_GETQDISC: u16 = 38;
pub const RTM_NEWTCLASS: u16 = 40;
pub const RTM_DELTCLASS: u16 = 41;
pub const RTM_GETTCLASS: u16 = 42;
pub const RTM_NEWTFILTER: u16 = 44;
pub const RTM_DELTFILTER: u16 = 45;
pub const RTM_GETTFILTER: u16 = 46;
pub const RTM_NEWACTION: u16 = 48;
pub const RTM_DELACTION: u16 = 49;
pub const RTM_GETACTION: u16 = 50;
pub const RTM_NEWPREFIX: u16 = 52;
pub const RTM_GETMULTICAST: u16 = 58;
pub const RTM_GETANYCAST: u16 = 62;
pub const RTM_NEWNEIGHTBL: u16 = 64;
pub const RTM_GETNEIGHTBL: u16 = 66;
pub const RTM_SETNEIGHTBL: u16 = 67;
pub const RTM_NEWNDUSEROPT: u16 = 68;
pub const RTM_NEWADDRLABEL: u16 = 72;
pub const RTM_DELADDRLABEL: u16 = 73;
pub const RTM_GETADDRLABEL: u16 = 74;
pub const RTM_GETDCB: u16 = 78;
pub const RTM_SETDCB: u16 = 79;
pub const RTM_NEWNETCONF: u16 = 80;
pub const RTM_DELNETCONF: u16 = 81;
pub const RTM_GETNETCONF: u16 = 82;
pub const RTM_NEWMDB: u16 = 84;
pub const RTM_DELMDB: u16 = 85;
pub const RTM_GETMDB: u16 = 86;
pub const RTM_NEWNSID: u16 = 88;
pub const RTM_DELNSID: u16 = 89;
pub const RTM_GETNSID: u16 = 90;
pub const RTM_NEWSTATS: u16 = 92;
pub const RTM_GETSTATS: u16 = 94;
pub const RTM_NEWCACHEREPORT: u16 = 96;

/// Unknown route
pub const RTN_UNSPEC: u8 = 0;
/// A gateway or direct route
pub const RTN_UNICAST: u8 = 1;
/// A local interface route
pub const RTN_LOCAL: u8 = 2;
/// A local broadcast route (sent as a broadcast)
pub const RTN_BROADCAST: u8 = 3;
/// A local broadcast route (sent as a unicast)
pub const RTN_ANYCAST: u8 = 4;
/// A multicast route
pub const RTN_MULTICAST: u8 = 5;
/// A packet dropping route
pub const RTN_BLACKHOLE: u8 = 6;
/// An unreachable destination
pub const RTN_UNREACHABLE: u8 = 7;
/// A packet rejection route
pub const RTN_PROHIBIT: u8 = 8;
/// Continue routing lookup in another table
pub const RTN_THROW: u8 = 9;
/// A network address translation rule
pub const RTN_NAT: u8 = 10;
/// Refer to an external resolver (not implemented)
pub const RTN_XRESOLVE: u8 = 11;

/// Unknown
pub const RTPROT_UNSPEC: u8 = 0;
/// Route was learnt by an ICMP redirect
pub const RTPROT_REDIRECT: u8 = 1;
/// Route was learnt by the kernel
pub const RTPROT_KERNEL: u8 = 2;
/// Route was learnt during boot
pub const RTPROT_BOOT: u8 = 3;
/// Route was set statically
pub const RTPROT_STATIC: u8 = 4;
pub const RTPROT_GATED: u8 = 8;
pub const RTPROT_RA: u8 = 9;
pub const RTPROT_MRT: u8 = 10;
pub const RTPROT_ZEBRA: u8 = 11;
pub const RTPROT_BIRD: u8 = 12;
pub const RTPROT_DNROUTED: u8 = 13;
pub const RTPROT_XORP: u8 = 14;
pub const RTPROT_NTK: u8 = 15;
pub const RTPROT_DHCP: u8 = 16;
pub const RTPROT_MROUTED: u8 = 17;
pub const RTPROT_BABEL: u8 = 42;

/// Global route
pub const RT_SCOPE_UNIVERSE: u8 = 0;
/// Interior route in the local autonomous system
pub const RT_SCOPE_SITE: u8 = 200;
/// Route on this link
pub const RT_SCOPE_LINK: u8 = 253;
/// Route on the local host
pub const RT_SCOPE_HOST: u8 = 254;
/// Destination doesn't exist
pub const RT_SCOPE_NOWHERE: u8 = 255;

pub const RT_TABLE_UNSPEC: u8 = 0;
pub const RT_TABLE_COMPAT: u8 = 252;
pub const RT_TABLE_DEFAULT: u8 = 253;
pub const RT_TABLE_MAIN: u8 = 254;
pub const RT_TABLE_LOCAL: u8 = 255;

pub const RTM_F_NOTIFY: u32 = 256;
pub const RTM_F_CLONED: u32 = 512;
pub const RTM_F_EQUALIZE: u32 = 1024;
pub const RTM_F_PREFIX: u32 = 2048;
pub const RTM_F_LOOKUP_TABLE: u32 = 4096;
pub const RTM_F_FIB_MATCH: u32 = 8192;

pub const AF_UNSPEC: u16 = libc::AF_UNSPEC as u16;
pub const AF_UNIX: u16 = libc::AF_UNIX as u16;
// pub const AF_LOCAL: u16 = libc::AF_LOCAL as u16;
pub const AF_INET: u16 = libc::AF_INET as u16;
pub const AF_AX25: u16 = libc::AF_AX25 as u16;
pub const AF_IPX: u16 = libc::AF_IPX as u16;
pub const AF_APPLETALK: u16 = libc::AF_APPLETALK as u16;
pub const AF_NETROM: u16 = libc::AF_NETROM as u16;
pub const AF_BRIDGE: u16 = libc::AF_BRIDGE as u16;
pub const AF_ATMPVC: u16 = libc::AF_ATMPVC as u16;
pub const AF_X25: u16 = libc::AF_X25 as u16;
pub const AF_INET6: u16 = libc::AF_INET6 as u16;
pub const AF_ROSE: u16 = libc::AF_ROSE as u16;
pub const AF_DECNET: u16 = libc::AF_DECnet as u16;
pub const AF_NETBEUI: u16 = libc::AF_NETBEUI as u16;
pub const AF_SECURITY: u16 = libc::AF_SECURITY as u16;
pub const AF_KEY: u16 = libc::AF_KEY as u16;
pub const AF_NETLINK: u16 = libc::AF_NETLINK as u16;
// pub const AF_ROUTE: u16 = libc::AF_ROUTE as u16;
pub const AF_PACKET: u16 = libc::AF_PACKET as u16;
pub const AF_ASH: u16 = libc::AF_ASH as u16;
pub const AF_ECONET: u16 = libc::AF_ECONET as u16;
pub const AF_ATMSVC: u16 = libc::AF_ATMSVC as u16;
pub const AF_RDS: u16 = libc::AF_RDS as u16;
pub const AF_SNA: u16 = libc::AF_SNA as u16;
pub const AF_IRDA: u16 = libc::AF_IRDA as u16;
pub const AF_PPPOX: u16 = libc::AF_PPPOX as u16;
pub const AF_WANPIPE: u16 = libc::AF_WANPIPE as u16;
pub const AF_LLC: u16 = libc::AF_LLC as u16;
pub const AF_CAN: u16 = libc::AF_CAN as u16;
pub const AF_TIPC: u16 = libc::AF_TIPC as u16;
pub const AF_BLUETOOTH: u16 = libc::AF_BLUETOOTH as u16;
pub const AF_IUCV: u16 = libc::AF_IUCV as u16;
pub const AF_RXRPC: u16 = libc::AF_RXRPC as u16;
pub const AF_ISDN: u16 = libc::AF_ISDN as u16;
pub const AF_PHONET: u16 = libc::AF_PHONET as u16;
pub const AF_IEEE802154: u16 = libc::AF_IEEE802154 as u16;
pub const AF_CAIF: u16 = libc::AF_CAIF as u16;
pub const AF_ALG: u16 = libc::AF_ALG as u16;

pub const NETNSA_NONE: u16 = 0;
pub const NETNSA_NSID: u16 = 1;
pub const NETNSA_PID: u16 = 2;
pub const NETNSA_FD: u16 = 3;
pub const NETNSA_NSID_NOT_ASSIGNED: i32 = -1;

/// Neighbour cache entry state: the neighbour has not (yet) been resolved
pub const NUD_INCOMPLETE: u16 = 1;
/// Neighbour cache entry state: the neighbour entry is valid until its lifetime expires
pub const NUD_REACHABLE: u16 = 2;
/// Neighbour cache entry state: the neighbour entry is valid but suspicious
pub const NUD_STALE: u16 = 4;
/// Neighbour cache entry state: the validation of this entry is currently delayed
pub const NUD_DELAY: u16 = 8;
/// Neighbour cache entry state: the neighbour entry is being probed
pub const NUD_PROBE: u16 = 16;
/// Neighbour cache entry state: the validation of this entry has failed
pub const NUD_FAILED: u16 = 32;
/// Neighbour cache entry state: entry is valid and the kernel will not try to validate or refresh it.
pub const NUD_NOARP: u16 = 64;
/// Neighbour cache entry state: entry is valid forever and can only be removed explicitly from userspace.
pub const NUD_PERMANENT: u16 = 128;
/// Neighbour cache entry state: pseudo state for fresh entries or before deleting entries
pub const NUD_NONE: u16 = 0;

// Neighbour cache entry flags
pub const NTF_USE: u8 = 1;
pub const NTF_SELF: u8 = 2;
pub const NTF_MASTER: u8 = 4;
pub const NTF_PROXY: u8 = 8;
pub const NTF_EXT_LEARNED: u8 = 16;
pub const NTF_OFFLOADED: u8 = 32;
pub const NTF_ROUTER: u8 = 128;

pub const TCA_UNSPEC: u16 = 0;
pub const TCA_KIND: u16 = 1;
pub const TCA_OPTIONS: u16 = 2;
pub const TCA_STATS: u16 = 3;
pub const TCA_XSTATS: u16 = 4;
pub const TCA_RATE: u16 = 5;
pub const TCA_FCNT: u16 = 6;
pub const TCA_STATS2: u16 = 7;
pub const TCA_STAB: u16 = 8;
pub const TCA_PAD: u16 = 9;
pub const TCA_DUMP_INVISIBLE: u16 = 10;
pub const TCA_CHAIN: u16 = 11;
pub const TCA_HW_OFFLOAD: u16 = 12;
pub const TCA_INGRESS_BLOCK: u16 = 13;
pub const TCA_EGRESS_BLOCK: u16 = 14;
pub const TCA_STATS_UNSPEC: u16 = 0;
pub const TCA_STATS_BASIC: u16 = 1;
pub const TCA_STATS_RATE_EST: u16 = 2;
pub const TCA_STATS_QUEUE: u16 = 3;
pub const TCA_STATS_APP: u16 = 4;
pub const TCA_STATS_RATE_EST64: u16 = 5;
pub const TCA_STATS_PAD: u16 = 6;
pub const TCA_STATS_BASIC_HW: u16 = 7;

pub const NDTA_UNSPEC: u16 = 0;
pub const NDTA_NAME: u16 = 1;
pub const NDTA_THRESH1: u16 = 2;
pub const NDTA_THRESH2: u16 = 3;
pub const NDTA_THRESH3: u16 = 4;
pub const NDTA_CONFIG: u16 = 5;
pub const NDTA_PARMS: u16 = 6;
pub const NDTA_STATS: u16 = 7;
pub const NDTA_GC_INTERVAL: u16 = 8;
pub const NDTA_PAD: u16 = 9;

pub const RTA_UNSPEC: u16 = 0;
pub const RTA_DST: u16 = 1;
pub const RTA_SRC: u16 = 2;
pub const RTA_IIF: u16 = 3;
pub const RTA_OIF: u16 = 4;
pub const RTA_GATEWAY: u16 = 5;
pub const RTA_PRIORITY: u16 = 6;
pub const RTA_PREFSRC: u16 = 7;
pub const RTA_METRICS: u16 = 8;
pub const RTA_MULTIPATH: u16 = 9;
pub const RTA_PROTOINFO: u16 = 10;
pub const RTA_FLOW: u16 = 11;
pub const RTA_CACHEINFO: u16 = 12;
pub const RTA_SESSION: u16 = 13;
pub const RTA_MP_ALGO: u16 = 14;
pub const RTA_TABLE: u16 = 15;
pub const RTA_MARK: u16 = 16;
pub const RTA_MFC_STATS: u16 = 17;
pub const RTA_VIA: u16 = 18;
pub const RTA_NEWDST: u16 = 19;
pub const RTA_PREF: u16 = 20;
pub const RTA_ENCAP_TYPE: u16 = 21;
pub const RTA_ENCAP: u16 = 22;
pub const RTA_EXPIRES: u16 = 23;
pub const RTA_PAD: u16 = 24;
pub const RTA_UID: u16 = 25;
pub const RTA_TTL_PROPAGATE: u16 = 26;

pub const RTAX_UNSPEC: u16 = 0;
pub const RTAX_LOCK: u16 = 1;
pub const RTAX_MTU: u16 = 2;
pub const RTAX_WINDOW: u16 = 3;
pub const RTAX_RTT: u16 = 4;
pub const RTAX_RTTVAR: u16 = 5;
pub const RTAX_SSTHRESH: u16 = 6;
pub const RTAX_CWND: u16 = 7;
pub const RTAX_ADVMSS: u16 = 8;
pub const RTAX_REORDERING: u16 = 9;
pub const RTAX_HOPLIMIT: u16 = 10;
pub const RTAX_INITCWND: u16 = 11;
pub const RTAX_FEATURES: u16 = 12;
pub const RTAX_RTO_MIN: u16 = 13;
pub const RTAX_INITRWND: u16 = 14;
pub const RTAX_QUICKACK: u16 = 15;
pub const RTAX_CC_ALGO: u16 = 16;
pub const RTAX_FASTOPEN_NO_COOKIE: u16 = 17;

pub const IFLA_INFO_UNSPEC: u16 = 0;
pub const IFLA_INFO_KIND: u16 = 1;
pub const IFLA_INFO_DATA: u16 = 2;
pub const IFLA_INFO_XSTATS: u16 = 3;
pub const IFLA_INFO_SLAVE_KIND: u16 = 4;
pub const IFLA_INFO_SLAVE_DATA: u16 = 5;
pub const IFLA_BR_UNSPEC: u16 = 0;
pub const IFLA_BR_FORWARD_DELAY: u16 = 1;
pub const IFLA_BR_HELLO_TIME: u16 = 2;
pub const IFLA_BR_MAX_AGE: u16 = 3;
pub const IFLA_BR_AGEING_TIME: u16 = 4;
pub const IFLA_BR_STP_STATE: u16 = 5;
pub const IFLA_BR_PRIORITY: u16 = 6;
pub const IFLA_BR_VLAN_FILTERING: u16 = 7;
pub const IFLA_BR_VLAN_PROTOCOL: u16 = 8;
pub const IFLA_BR_GROUP_FWD_MASK: u16 = 9;
pub const IFLA_BR_ROOT_ID: u16 = 10;
pub const IFLA_BR_BRIDGE_ID: u16 = 11;
pub const IFLA_BR_ROOT_PORT: u16 = 12;
pub const IFLA_BR_ROOT_PATH_COST: u16 = 13;
pub const IFLA_BR_TOPOLOGY_CHANGE: u16 = 14;
pub const IFLA_BR_TOPOLOGY_CHANGE_DETECTED: u16 = 15;
pub const IFLA_BR_HELLO_TIMER: u16 = 16;
pub const IFLA_BR_TCN_TIMER: u16 = 17;
pub const IFLA_BR_TOPOLOGY_CHANGE_TIMER: u16 = 18;
pub const IFLA_BR_GC_TIMER: u16 = 19;
pub const IFLA_BR_GROUP_ADDR: u16 = 20;
pub const IFLA_BR_FDB_FLUSH: u16 = 21;
pub const IFLA_BR_MCAST_ROUTER: u16 = 22;
pub const IFLA_BR_MCAST_SNOOPING: u16 = 23;
pub const IFLA_BR_MCAST_QUERY_USE_IFADDR: u16 = 24;
pub const IFLA_BR_MCAST_QUERIER: u16 = 25;
pub const IFLA_BR_MCAST_HASH_ELASTICITY: u16 = 26;
pub const IFLA_BR_MCAST_HASH_MAX: u16 = 27;
pub const IFLA_BR_MCAST_LAST_MEMBER_CNT: u16 = 28;
pub const IFLA_BR_MCAST_STARTUP_QUERY_CNT: u16 = 29;
pub const IFLA_BR_MCAST_LAST_MEMBER_INTVL: u16 = 30;
pub const IFLA_BR_MCAST_MEMBERSHIP_INTVL: u16 = 31;
pub const IFLA_BR_MCAST_QUERIER_INTVL: u16 = 32;
pub const IFLA_BR_MCAST_QUERY_INTVL: u16 = 33;
pub const IFLA_BR_MCAST_QUERY_RESPONSE_INTVL: u16 = 34;
pub const IFLA_BR_MCAST_STARTUP_QUERY_INTVL: u16 = 35;
pub const IFLA_BR_NF_CALL_IPTABLES: u16 = 36;
pub const IFLA_BR_NF_CALL_IP6TABLES: u16 = 37;
pub const IFLA_BR_NF_CALL_ARPTABLES: u16 = 38;
pub const IFLA_BR_VLAN_DEFAULT_PVID: u16 = 39;
pub const IFLA_BR_PAD: u16 = 40;
pub const IFLA_BR_VLAN_STATS_ENABLED: u16 = 41;
pub const IFLA_BR_MCAST_STATS_ENABLED: u16 = 42;
pub const IFLA_BR_MCAST_IGMP_VERSION: u16 = 43;
pub const IFLA_BR_MCAST_MLD_VERSION: u16 = 44;
pub const IFLA_VLAN_UNSPEC: u16 = 0;
pub const IFLA_VLAN_ID: u16 = 1;
pub const IFLA_VLAN_FLAGS: u16 = 2;
pub const IFLA_VLAN_EGRESS_QOS: u16 = 3;
pub const IFLA_VLAN_INGRESS_QOS: u16 = 4;
pub const IFLA_VLAN_PROTOCOL: u16 = 5;
pub const VETH_INFO_UNSPEC: u16 = 0;
pub const VETH_INFO_PEER: u16 = 1;

pub const ARPHRD_NETROM: u16 = 0;
pub const ARPHRD_ETHER: u16 = 1;
pub const ARPHRD_EETHER: u16 = 2;
pub const ARPHRD_AX25: u16 = 3;
pub const ARPHRD_PRONET: u16 = 4;
pub const ARPHRD_CHAOS: u16 = 5;
pub const ARPHRD_IEEE802: u16 = 6;
pub const ARPHRD_ARCNET: u16 = 7;
pub const ARPHRD_APPLETLK: u16 = 8;
pub const ARPHRD_DLCI: u16 = 15;
pub const ARPHRD_ATM: u16 = 19;
pub const ARPHRD_METRICOM: u16 = 23;
pub const ARPHRD_IEEE1394: u16 = 24;
pub const ARPHRD_EUI64: u16 = 27;
pub const ARPHRD_INFINIBAND: u16 = 32;
pub const ARPHRD_SLIP: u16 = 256;
pub const ARPHRD_CSLIP: u16 = 257;
pub const ARPHRD_SLIP6: u16 = 258;
pub const ARPHRD_CSLIP6: u16 = 259;
pub const ARPHRD_RSRVD: u16 = 260;
pub const ARPHRD_ADAPT: u16 = 264;
pub const ARPHRD_ROSE: u16 = 270;
pub const ARPHRD_X25: u16 = 271;
pub const ARPHRD_HWX25: u16 = 272;
pub const ARPHRD_CAN: u16 = 280;
pub const ARPHRD_PPP: u16 = 512;
pub const ARPHRD_CISCO: u16 = 513;
pub const ARPHRD_HDLC: u16 = 513;
pub const ARPHRD_LAPB: u16 = 516;
pub const ARPHRD_DDCMP: u16 = 517;
pub const ARPHRD_RAWHDLC: u16 = 518;
pub const ARPHRD_RAWIP: u16 = 519;
pub const ARPHRD_TUNNEL: u16 = 768;
pub const ARPHRD_TUNNEL6: u16 = 769;
pub const ARPHRD_FRAD: u16 = 770;
pub const ARPHRD_SKIP: u16 = 771;
pub const ARPHRD_LOOPBACK: u16 = 772;
pub const ARPHRD_LOCALTLK: u16 = 773;
pub const ARPHRD_FDDI: u16 = 774;
pub const ARPHRD_BIF: u16 = 775;
pub const ARPHRD_SIT: u16 = 776;
pub const ARPHRD_IPDDP: u16 = 777;
pub const ARPHRD_IPGRE: u16 = 778;
pub const ARPHRD_PIMREG: u16 = 779;
pub const ARPHRD_HIPPI: u16 = 780;
pub const ARPHRD_ASH: u16 = 781;
pub const ARPHRD_ECONET: u16 = 782;
pub const ARPHRD_IRDA: u16 = 783;
pub const ARPHRD_FCPP: u16 = 784;
pub const ARPHRD_FCAL: u16 = 785;
pub const ARPHRD_FCPL: u16 = 786;
pub const ARPHRD_FCFABRIC: u16 = 787;
pub const ARPHRD_IEEE802_TR: u16 = 800;
pub const ARPHRD_IEEE80211: u16 = 801;
pub const ARPHRD_IEEE80211_PRISM: u16 = 802;
pub const ARPHRD_IEEE80211_RADIOTAP: u16 = 803;
pub const ARPHRD_IEEE802154: u16 = 804;
pub const ARPHRD_IEEE802154_MONITOR: u16 = 805;
pub const ARPHRD_PHONET: u16 = 820;
pub const ARPHRD_PHONET_PIPE: u16 = 821;
pub const ARPHRD_CAIF: u16 = 822;
pub const ARPHRD_IP6GRE: u16 = 823;
pub const ARPHRD_NETLINK: u16 = 824;
pub const ARPHRD_6LOWPAN: u16 = 825;
pub const ARPHRD_VSOCKMON: u16 = 826;
pub const ARPHRD_VOID: u16 = 65535;
pub const ARPHRD_NONE: u16 = 65534;

pub const IFA_UNSPEC: u16 = 0;
pub const IFA_ADDRESS: u16 = 1;
pub const IFA_LOCAL: u16 = 2;
pub const IFA_LABEL: u16 = 3;
pub const IFA_BROADCAST: u16 = 4;
pub const IFA_ANYCAST: u16 = 5;
pub const IFA_CACHEINFO: u16 = 6;
pub const IFA_MULTICAST: u16 = 7;
pub const IFA_FLAGS: u16 = 8;

pub const IFLA_UNSPEC: u16 = 0;
pub const IFLA_ADDRESS: u16 = 1;
pub const IFLA_BROADCAST: u16 = 2;
pub const IFLA_IFNAME: u16 = 3;
pub const IFLA_MTU: u16 = 4;
pub const IFLA_LINK: u16 = 5;
pub const IFLA_QDISC: u16 = 6;
pub const IFLA_STATS: u16 = 7;
pub const IFLA_COST: u16 = 8;
pub const IFLA_PRIORITY: u16 = 9;
pub const IFLA_MASTER: u16 = 10;
pub const IFLA_WIRELESS: u16 = 11;
pub const IFLA_PROTINFO: u16 = 12;
pub const IFLA_TXQLEN: u16 = 13;
pub const IFLA_MAP: u16 = 14;
pub const IFLA_WEIGHT: u16 = 15;
pub const IFLA_OPERSTATE: u16 = 16;
pub const IFLA_LINKMODE: u16 = 17;
pub const IFLA_LINKINFO: u16 = 18;
pub const IFLA_NET_NS_PID: u16 = 19;
pub const IFLA_IFALIAS: u16 = 20;
pub const IFLA_NUM_VF: u16 = 21;
pub const IFLA_VFINFO_LIST: u16 = 22;
pub const IFLA_STATS64: u16 = 23;
pub const IFLA_VF_PORTS: u16 = 24;
pub const IFLA_PORT_SELF: u16 = 25;
pub const IFLA_AF_SPEC: u16 = 26;
pub const IFLA_GROUP: u16 = 27;
pub const IFLA_NET_NS_FD: u16 = 28;
pub const IFLA_EXT_MASK: u16 = 29;
pub const IFLA_PROMISCUITY: u16 = 30;
pub const IFLA_NUM_TX_QUEUES: u16 = 31;
pub const IFLA_NUM_RX_QUEUES: u16 = 32;
pub const IFLA_CARRIER: u16 = 33;
pub const IFLA_PHYS_PORT_ID: u16 = 34;
pub const IFLA_CARRIER_CHANGES: u16 = 35;
pub const IFLA_PHYS_SWITCH_ID: u16 = 36;
pub const IFLA_LINK_NETNSID: u16 = 37;
pub const IFLA_PHYS_PORT_NAME: u16 = 38;
pub const IFLA_PROTO_DOWN: u16 = 39;
pub const IFLA_GSO_MAX_SEGS: u16 = 40;
pub const IFLA_GSO_MAX_SIZE: u16 = 41;
pub const IFLA_PAD: u16 = 42;
pub const IFLA_XDP: u16 = 43;
pub const IFLA_EVENT: u16 = 44;
pub const IFLA_NEW_NETNSID: u16 = 45;
pub const IFLA_IF_NETNSID: u16 = 46;
pub const IFLA_CARRIER_UP_COUNT: u16 = 47;
pub const IFLA_CARRIER_DOWN_COUNT: u16 = 48;
pub const IFLA_NEW_IFINDEX: u16 = 49;
pub const IFLA_INET_UNSPEC: u16 = 0;
pub const IFLA_INET_CONF: u16 = 1;
pub const IFLA_INET6_UNSPEC: u16 = 0;
pub const IFLA_INET6_FLAGS: u16 = 1;
pub const IFLA_INET6_CONF: u16 = 2;
pub const IFLA_INET6_STATS: u16 = 3;
// pub const IFLA_INET6_MCAST: u16 = 4;
pub const IFLA_INET6_CACHEINFO: u16 = 5;
pub const IFLA_INET6_ICMP6STATS: u16 = 6;
pub const IFLA_INET6_TOKEN: u16 = 7;
pub const IFLA_INET6_ADDR_GEN_MODE: u16 = 8;

/// Link is up (administratively).
pub const IFF_UP: u32 = libc::IFF_UP as u32;
/// Link is up and carrier is OK (RFC2863 OPER_UP)
pub const IFF_RUNNING: u32 = libc::IFF_RUNNING as u32;
/// Link layer is operational
pub const IFF_LOWER_UP: u32 = libc::IFF_LOWER_UP as u32;
/// Driver signals IFF_DORMANT
pub const IFF_DORMANT: u32 = libc::IFF_DORMANT as u32;
/// Link supports broadcasting
pub const IFF_BROADCAST: u32 = libc::IFF_BROADCAST as u32;
/// Link supports multicasting
pub const IFF_MULTICAST: u32 = libc::IFF_MULTICAST as u32;
/// Link supports multicast routing
pub const IFF_ALLMULTI: u32 = libc::IFF_ALLMULTI as u32;
/// Tell driver to do debugging (currently unused)
pub const IFF_DEBUG: u32 = libc::IFF_DEBUG as u32;
/// Link loopback network
pub const IFF_LOOPBACK: u32 = libc::IFF_LOOPBACK as u32;
/// u32erface is point-to-point link
pub const IFF_POINTOPOINT: u32 = libc::IFF_POINTOPOINT as u32;
/// ARP is not supported
pub const IFF_NOARP: u32 = libc::IFF_NOARP as u32;
/// Receive all packets.
pub const IFF_PROMISC: u32 = libc::IFF_PROMISC as u32;
/// Master of a load balancer (bonding)
pub const IFF_MASTER: u32 = libc::IFF_MASTER as u32;
/// Slave of a load balancer
pub const IFF_SLAVE: u32 = libc::IFF_SLAVE as u32;
/// Link selects port automatically (only used by ARM ethernet)
pub const IFF_PORTSEL: u32 = libc::IFF_PORTSEL as u32;
/// Driver supports setting media type (only used by ARM ethernet)
pub const IFF_AUTOMEDIA: u32 = libc::IFF_AUTOMEDIA as u32;
// /// Echo sent packets (testing feature, CAN only)
// pub const IFF_ECHO: u32 = libc::IFF_ECHO as u32;
// /// Dialup device with changing addresses (unused, BSD compatibility)
// pub const IFF_DYNAMIC: u32 = libc::IFF_DYNAMIC as u32;
// /// Avoid use of trailers (unused, BSD compatibility)
// pub const IFF_NOTRAILERS: u32 = libc::IFF_NOTRAILERS as u32;

pub const IF_OPER_UNKNOWN: u8 = 0;
pub const IF_OPER_NOTPRESENT: u8 = 1;
pub const IF_OPER_DOWN: u8 = 2;
pub const IF_OPER_LOWERLAYERDOWN: u8 = 3;
pub const IF_OPER_TESTING: u8 = 4;
pub const IF_OPER_DORMANT: u8 = 5;
pub const IF_OPER_UP: u8 = 6;

/// Neighbour cache entry type: unknown type
pub const NDA_UNSPEC: u16 = 0;
/// Neighbour cache entry type: entry for a network layer destination
/// address
pub const NDA_DST: u16 = 1;
/// Neighbour cache entry type: entry for a link layer destination
/// address
pub const NDA_LLADDR: u16 = 2;
/// Neighbour cache entry type: entry for cache statistics
pub const NDA_CACHEINFO: u16 = 3;
pub const NDA_PROBES: u16 = 4;
pub const NDA_VLAN: u16 = 5;
pub const NDA_PORT: u16 = 6;
pub const NDA_VNI: u16 = 7;
pub const NDA_IFINDEX: u16 = 8;
pub const NDA_MASTER: u16 = 9;
pub const NDA_LINK_NETNSID: u16 = 10;
pub const NDA_SRC_VNI: u16 = 11;
