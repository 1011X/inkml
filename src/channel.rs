use xsd;

enum ChannelKind {
	Integer(xsd::decimal),
	Decimal(xsd::decimal),
	Boolean(xsd::boolean),
}

struct Channel {
	/// The unique identifier for this element.
	/// Required: no, Default: none
	id: Option<xsd::ID>,
	
	/// The case sensitive name of this channel.
	/// Required: yes
	name: xsd::string,
	
	/// The data type of the point values and the default value of the point
	/// data for this channel. Default value only applies to intermittent
	/// channels.
	/// Required: no, Default: "decimal"
	kind: ChannelKind,
	
	/// The lower boundary for the values of this channel.
	/// Required: no, Default: none
	min: Option<xsd::number>,
	
	/// The upper boundary for the values of this channel.
	/// Required: no, Default: none
	max: Option<xsd::number>,
	
	/// The orientation of increasing channel values with respect to the default
	/// direction of the channel's coordinate axis, where applicable.
	/// Required: no, Default: "+ve"
	orientation: Orientation,
	
	/// Specifies that the values are relative to another reference point. The
	/// reference point may be the URI of a <timestamp> for time channels, or an
	/// application defined URI for application specific channels.
	/// Required: no, Default: none
	respectTo: Option<xsd::anyURI>,
	
	/// The units in which the values of the channel are expressed (numerical channels only).
	/// Required: no, Default: none
	units: Option<xsd::string>,
}
