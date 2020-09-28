#import "BridgeFfiPlugin.h"
#if __has_include(<bridge_ffi/bridge_ffi-Swift.h>)
#import <bridge_ffi/bridge_ffi-Swift.h>
#else
// Support project import fallback if the generated compatibility header
// is not copied when this plugin is created as a library.
// https://forums.swift.org/t/swift-static-libraries-dont-copy-generated-objective-c-header/19816
#import "bridge_ffi-Swift.h"
#endif

@implementation BridgeFfiPlugin
+ (void)registerWithRegistrar:(NSObject<FlutterPluginRegistrar>*)registrar {
  [SwiftBridgeFfiPlugin registerWithRegistrar:registrar];
}
@end
