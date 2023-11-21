// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

/// See the [Rust documentation for `LineBreakIterator`](https://docs.rs/icu/latest/icu/segmenter/struct.LineBreakIterator.html) for more information.
///
/// Additional information: [1](https://docs.rs/icu/latest/icu/segmenter/type.LineBreakIteratorPotentiallyIllFormedUtf8.html)
class LineBreakIteratorUtf8 implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  LineBreakIteratorUtf8._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XLineBreakIteratorUtf8_destroy'));

  /// Finds the next breakpoint. Returns -1 if at the end of the string or if the index is
  /// out of range of a 32-bit signed integer.
  ///
  /// See the [Rust documentation for `next`](https://docs.rs/icu/latest/icu/segmenter/struct.LineBreakIterator.html#method.next) for more information.
  int next() {
    final result = _ICU4XLineBreakIteratorUtf8_next(_underlying);
    return result;
  }

  // ignore: non_constant_identifier_names
  static final _ICU4XLineBreakIteratorUtf8_next =
      _capi<ffi.NativeFunction<ffi.Int32 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XLineBreakIteratorUtf8_next')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}