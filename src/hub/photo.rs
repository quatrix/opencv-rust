//! # Computational Photography
//! 
//! This module includes photo processing algorithms
//! # Inpainting
//! # Denoising
//! # HDR imaging
//! 
//! This section describes high dynamic range imaging algorithms namely tonemapping, exposure alignment,
//! camera calibration with multiple exposures and exposure fusion.
//! 
//! # Contrast Preserving Decolorization
//! 
//! Useful links:
//! 
//! http://www.cse.cuhk.edu.hk/leojia/projects/color2gray/index.html
//! 
//! # Seamless Cloning
//! 
//! Useful links:
//! 
//! https://www.learnopencv.com/seamless-cloning-using-opencv-python-cpp
//! 
//! # Non-Photorealistic Rendering
//! 
//! Useful links:
//! 
//! http://www.inf.ufrgs.br/~eslgastal/DomainTransform
//! 
//! https://www.learnopencv.com/non-photorealistic-rendering-using-opencv-python-c/
use std::os::raw::{c_char, c_void};
use libc::size_t;
use crate::{Error, Result, core, sys, types};

pub const CV_INPAINT_NS: i32 = 0;
pub const CV_INPAINT_TELEA: i32 = 1;
pub const INPAINT_NS: i32 = 0;
pub const INPAINT_TELEA: i32 = 1;
pub const LDR_SIZE: i32 = 256;
pub const MIXED_CLONE: i32 = 2;
pub const MONOCHROME_TRANSFER: i32 = 3;
pub const NORMAL_CLONE: i32 = 1;
pub const NORMCONV_FILTER: i32 = 2;
pub const RECURS_FILTER: i32 = 1;

// identifier: cv_colorChange_Mat_src_Mat_mask_Mat_dst_float_red_mul_float_green_mul_float_blue_mul
/// Given an original color image, two differently colored versions of this image can be mixed
/// seamlessly.
/// 
/// ## Parameters
/// * src: Input 8-bit 3-channel image.
/// * mask: Input 8-bit 1 or 3-channel image.
/// * dst: Output image with the same size and type as src .
/// * red_mul: R-channel multiply factor.
/// * green_mul: G-channel multiply factor.
/// * blue_mul: B-channel multiply factor.
/// 
/// Multiplication factor is between .5 to 2.5.
///
/// ## C++ default parameters:
/// * red_mul: 1.0f
/// * green_mul: 1.0f
/// * blue_mul: 1.0f
pub fn color_change(src: &core::Mat, mask: &core::Mat, dst: &mut core::Mat, red_mul: f32, green_mul: f32, blue_mul: f32) -> Result<()> {
    unsafe { sys::cv_photo_cv_colorChange_Mat_src_Mat_mask_Mat_dst_float_red_mul_float_green_mul_float_blue_mul(src.as_raw_Mat(), mask.as_raw_Mat(), dst.as_raw_Mat(), red_mul, green_mul, blue_mul) }.into_result()
}

// identifier: cv_createAlignMTB_int_max_bits_int_exclude_range_bool_cut
/// Creates AlignMTB object
/// 
/// ## Parameters
/// * max_bits: logarithm to the base 2 of maximal shift in each dimension. Values of 5 and 6 are
/// usually good enough (31 and 63 pixels shift respectively).
/// * exclude_range: range for exclusion bitmap that is constructed to suppress noise around the
/// median value.
/// * cut: if true cuts images, otherwise fills the new regions with zeros.
///
/// ## C++ default parameters:
/// * max_bits: 6
/// * exclude_range: 4
/// * cut: true
pub fn create_align_mtb(max_bits: i32, exclude_range: i32, cut: bool) -> Result<types::PtrOfAlignMTB> {
    unsafe { sys::cv_photo_cv_createAlignMTB_int_max_bits_int_exclude_range_bool_cut(max_bits, exclude_range, cut) }.into_result().map(|x| types::PtrOfAlignMTB { ptr: x })
}

// identifier: cv_createCalibrateDebevec_int_samples_float_lambda_bool_random
/// Creates CalibrateDebevec object
/// 
/// ## Parameters
/// * samples: number of pixel locations to use
/// * lambda: smoothness term weight. Greater values produce smoother results, but can alter the
/// response.
/// * random: if true sample pixel locations are chosen at random, otherwise they form a
/// rectangular grid.
///
/// ## C++ default parameters:
/// * samples: 70
/// * lambda: 10.0f
/// * random: false
pub fn create_calibrate_debevec(samples: i32, lambda: f32, random: bool) -> Result<types::PtrOfCalibrateDebevec> {
    unsafe { sys::cv_photo_cv_createCalibrateDebevec_int_samples_float_lambda_bool_random(samples, lambda, random) }.into_result().map(|x| types::PtrOfCalibrateDebevec { ptr: x })
}

// identifier: cv_createCalibrateRobertson_int_max_iter_float_threshold
/// Creates CalibrateRobertson object
/// 
/// ## Parameters
/// * max_iter: maximal number of Gauss-Seidel solver iterations.
/// * threshold: target difference between results of two successive steps of the minimization.
///
/// ## C++ default parameters:
/// * max_iter: 30
/// * threshold: 0.01f
pub fn create_calibrate_robertson(max_iter: i32, threshold: f32) -> Result<types::PtrOfCalibrateRobertson> {
    unsafe { sys::cv_photo_cv_createCalibrateRobertson_int_max_iter_float_threshold(max_iter, threshold) }.into_result().map(|x| types::PtrOfCalibrateRobertson { ptr: x })
}

// identifier: cv_createMergeDebevec
/// Creates MergeDebevec object
pub fn create_merge_debevec() -> Result<types::PtrOfMergeDebevec> {
    unsafe { sys::cv_photo_cv_createMergeDebevec() }.into_result().map(|x| types::PtrOfMergeDebevec { ptr: x })
}

// identifier: cv_createMergeMertens_float_contrast_weight_float_saturation_weight_float_exposure_weight
/// Creates MergeMertens object
/// 
/// ## Parameters
/// * contrast_weight: contrast measure weight. See MergeMertens.
/// * saturation_weight: saturation measure weight
/// * exposure_weight: well-exposedness measure weight
///
/// ## C++ default parameters:
/// * contrast_weight: 1.0f
/// * saturation_weight: 1.0f
/// * exposure_weight: 0.0f
pub fn create_merge_mertens(contrast_weight: f32, saturation_weight: f32, exposure_weight: f32) -> Result<types::PtrOfMergeMertens> {
    unsafe { sys::cv_photo_cv_createMergeMertens_float_contrast_weight_float_saturation_weight_float_exposure_weight(contrast_weight, saturation_weight, exposure_weight) }.into_result().map(|x| types::PtrOfMergeMertens { ptr: x })
}

// identifier: cv_createMergeRobertson
/// Creates MergeRobertson object
pub fn create_merge_robertson() -> Result<types::PtrOfMergeRobertson> {
    unsafe { sys::cv_photo_cv_createMergeRobertson() }.into_result().map(|x| types::PtrOfMergeRobertson { ptr: x })
}

// identifier: cv_createTonemapDrago_float_gamma_float_saturation_float_bias
/// Creates TonemapDrago object
/// 
/// ## Parameters
/// * gamma: gamma value for gamma correction. See createTonemap
/// * saturation: positive saturation enhancement value. 1.0 preserves saturation, values greater
/// than 1 increase saturation and values less than 1 decrease it.
/// * bias: value for bias function in [0, 1] range. Values from 0.7 to 0.9 usually give best
/// results, default value is 0.85.
///
/// ## C++ default parameters:
/// * gamma: 1.0f
/// * saturation: 1.0f
/// * bias: 0.85f
pub fn create_tonemap_drago(gamma: f32, saturation: f32, bias: f32) -> Result<types::PtrOfTonemapDrago> {
    unsafe { sys::cv_photo_cv_createTonemapDrago_float_gamma_float_saturation_float_bias(gamma, saturation, bias) }.into_result().map(|x| types::PtrOfTonemapDrago { ptr: x })
}

// identifier: cv_createTonemapMantiuk_float_gamma_float_scale_float_saturation
/// Creates TonemapMantiuk object
/// 
/// ## Parameters
/// * gamma: gamma value for gamma correction. See createTonemap
/// * scale: contrast scale factor. HVS response is multiplied by this parameter, thus compressing
/// dynamic range. Values from 0.6 to 0.9 produce best results.
/// * saturation: saturation enhancement value. See createTonemapDrago
///
/// ## C++ default parameters:
/// * gamma: 1.0f
/// * scale: 0.7f
/// * saturation: 1.0f
pub fn create_tonemap_mantiuk(gamma: f32, scale: f32, saturation: f32) -> Result<types::PtrOfTonemapMantiuk> {
    unsafe { sys::cv_photo_cv_createTonemapMantiuk_float_gamma_float_scale_float_saturation(gamma, scale, saturation) }.into_result().map(|x| types::PtrOfTonemapMantiuk { ptr: x })
}

// identifier: cv_createTonemapReinhard_float_gamma_float_intensity_float_light_adapt_float_color_adapt
/// Creates TonemapReinhard object
/// 
/// ## Parameters
/// * gamma: gamma value for gamma correction. See createTonemap
/// * intensity: result intensity in [-8, 8] range. Greater intensity produces brighter results.
/// * light_adapt: light adaptation in [0, 1] range. If 1 adaptation is based only on pixel
/// value, if 0 it's global, otherwise it's a weighted mean of this two cases.
/// * color_adapt: chromatic adaptation in [0, 1] range. If 1 channels are treated independently,
/// if 0 adaptation level is the same for each channel.
///
/// ## C++ default parameters:
/// * gamma: 1.0f
/// * intensity: 0.0f
/// * light_adapt: 1.0f
/// * color_adapt: 0.0f
pub fn create_tonemap_reinhard(gamma: f32, intensity: f32, light_adapt: f32, color_adapt: f32) -> Result<types::PtrOfTonemapReinhard> {
    unsafe { sys::cv_photo_cv_createTonemapReinhard_float_gamma_float_intensity_float_light_adapt_float_color_adapt(gamma, intensity, light_adapt, color_adapt) }.into_result().map(|x| types::PtrOfTonemapReinhard { ptr: x })
}

// identifier: cv_createTonemap_float_gamma
/// Creates simple linear mapper with gamma correction
/// 
/// ## Parameters
/// * gamma: positive value for gamma correction. Gamma value of 1.0 implies no correction, gamma
/// equal to 2.2f is suitable for most displays.
/// Generally gamma \> 1 brightens the image and gamma \< 1 darkens it.
///
/// ## C++ default parameters:
/// * gamma: 1.0f
pub fn create_tonemap(gamma: f32) -> Result<types::PtrOfTonemap> {
    unsafe { sys::cv_photo_cv_createTonemap_float_gamma(gamma) }.into_result().map(|x| types::PtrOfTonemap { ptr: x })
}

// identifier: cv_decolor_Mat_src_Mat_grayscale_Mat_color_boost
/// Transforms a color image to a grayscale image. It is a basic tool in digital printing, stylized
/// black-and-white photograph rendering, and in many single channel image processing applications
/// [CL12](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_CL12) .
/// 
/// ## Parameters
/// * src: Input 8-bit 3-channel image.
/// * grayscale: Output 8-bit 1-channel image.
/// * color_boost: Output 8-bit 3-channel image.
/// 
/// This function is to be applied on color images.
pub fn decolor(src: &core::Mat, grayscale: &mut core::Mat, color_boost: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_photo_cv_decolor_Mat_src_Mat_grayscale_Mat_color_boost(src.as_raw_Mat(), grayscale.as_raw_Mat(), color_boost.as_raw_Mat()) }.into_result()
}

// identifier: cv_denoise_TVL1_VectorOfMat_observations_Mat_result_double_lambda_int_niters
/// Primal-dual algorithm is an algorithm for solving special types of variational problems (that is,
/// finding a function to minimize some functional). As the image denoising, in particular, may be seen
/// as the variational problem, primal-dual algorithm then can be used to perform denoising and this is
/// exactly what is implemented.
/// 
/// It should be noted, that this implementation was taken from the July 2013 blog entry
/// [MA13](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_MA13) , which also contained (slightly more general) ready-to-use source code on Python.
/// Subsequently, that code was rewritten on C++ with the usage of openCV by Vadim Pisarevsky at the end
/// of July 2013 and finally it was slightly adapted by later authors.
/// 
/// Although the thorough discussion and justification of the algorithm involved may be found in
/// [ChambolleEtAl](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_ChambolleEtAl), it might make sense to skim over it here, following [MA13](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_MA13) . To begin
/// with, we consider the 1-byte gray-level images as the functions from the rectangular domain of
/// pixels (it may be seen as set
/// <span lang='latex'>\left\{(x,y)\in\mathbb{N}\times\mathbb{N}\mid 1\leq x\leq n,\;1\leq y\leq m\right\}</span> for some
/// <span lang='latex'>m,\;n\in\mathbb{N}</span>) into <span lang='latex'>\{0,1,\dots,255\}</span>. We shall denote the noised images as <span lang='latex'>f_i</span> and with
/// this view, given some image <span lang='latex'>x</span> of the same size, we may measure how bad it is by the formula
/// 
/// <div lang='latex'>\left\|\left\|\nabla x\right\|\right\| + \lambda\sum_i\left\|\left\|x-f_i\right\|\right\|</div>
/// 
/// <span lang='latex'>\|\|\cdot\|\|</span> here denotes <span lang='latex'>L_2</span>-norm and as you see, the first addend states that we want our
/// image to be smooth (ideally, having zero gradient, thus being constant) and the second states that
/// we want our result to be close to the observations we've got. If we treat <span lang='latex'>x</span> as a function, this is
/// exactly the functional what we seek to minimize and here the Primal-Dual algorithm comes into play.
/// 
/// ## Parameters
/// * observations: This array should contain one or more noised versions of the image that is to
/// be restored.
/// * result: Here the denoised image will be stored. There is no need to do pre-allocation of
/// storage space, as it will be automatically allocated, if necessary.
/// * lambda: Corresponds to <span lang='latex'>\lambda</span> in the formulas above. As it is enlarged, the smooth
/// (blurred) images are treated more favorably than detailed (but maybe more noised) ones. Roughly
/// speaking, as it becomes smaller, the result will be more blur but more sever outliers will be
/// removed.
/// * niters: Number of iterations that the algorithm will run. Of course, as more iterations as
/// better, but it is hard to quantitatively refine this statement, so just use the default and
/// increase it if the results are poor.
///
/// ## C++ default parameters:
/// * lambda: 1.0
/// * niters: 30
pub fn denoise_tvl1(observations: &types::VectorOfMat, result: &core::Mat, lambda: f64, niters: i32) -> Result<()> {
    unsafe { sys::cv_photo_cv_denoise_TVL1_VectorOfMat_observations_Mat_result_double_lambda_int_niters(observations.as_raw_VectorOfMat(), result.as_raw_Mat(), lambda, niters) }.into_result()
}

// identifier: cv_detailEnhance_Mat_src_Mat_dst_float_sigma_s_float_sigma_r
/// This filter enhances the details of a particular image.
/// 
/// ## Parameters
/// * src: Input 8-bit 3-channel image.
/// * dst: Output image with the same size and type as src.
/// * sigma_s: %Range between 0 to 200.
/// * sigma_r: %Range between 0 to 1.
///
/// ## C++ default parameters:
/// * sigma_s: 10
/// * sigma_r: 0.15f
pub fn detail_enhance(src: &core::Mat, dst: &mut core::Mat, sigma_s: f32, sigma_r: f32) -> Result<()> {
    unsafe { sys::cv_photo_cv_detailEnhance_Mat_src_Mat_dst_float_sigma_s_float_sigma_r(src.as_raw_Mat(), dst.as_raw_Mat(), sigma_s, sigma_r) }.into_result()
}

// identifier: cv_edgePreservingFilter_Mat_src_Mat_dst_int_flags_float_sigma_s_float_sigma_r
/// Filtering is the fundamental operation in image and video processing. Edge-preserving smoothing
/// filters are used in many different applications [EM11](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_EM11) .
/// 
/// ## Parameters
/// * src: Input 8-bit 3-channel image.
/// * dst: Output 8-bit 3-channel image.
/// * flags: Edge preserving filters: cv::RECURS_FILTER or cv::NORMCONV_FILTER
/// * sigma_s: %Range between 0 to 200.
/// * sigma_r: %Range between 0 to 1.
///
/// ## C++ default parameters:
/// * flags: 1
/// * sigma_s: 60
/// * sigma_r: 0.4f
pub fn edge_preserving_filter(src: &core::Mat, dst: &mut core::Mat, flags: i32, sigma_s: f32, sigma_r: f32) -> Result<()> {
    unsafe { sys::cv_photo_cv_edgePreservingFilter_Mat_src_Mat_dst_int_flags_float_sigma_s_float_sigma_r(src.as_raw_Mat(), dst.as_raw_Mat(), flags, sigma_s, sigma_r) }.into_result()
}

// identifier: cv_fastNlMeansDenoisingColoredMulti_VectorOfMat_srcImgs_Mat_dst_int_imgToDenoiseIndex_int_temporalWindowSize_float_h_float_hColor_int_templateWindowSize_int_searchWindowSize
/// Modification of fastNlMeansDenoisingMulti function for colored images sequences
/// 
/// ## Parameters
/// * srcImgs: Input 8-bit 3-channel images sequence. All images should have the same type and
/// size.
/// * imgToDenoiseIndex: Target image to denoise index in srcImgs sequence
/// * temporalWindowSize: Number of surrounding images to use for target image denoising. Should
/// be odd. Images from imgToDenoiseIndex - temporalWindowSize / 2 to
/// imgToDenoiseIndex - temporalWindowSize / 2 from srcImgs will be used to denoise
/// srcImgs[imgToDenoiseIndex] image.
/// * dst: Output image with the same size and type as srcImgs images.
/// * templateWindowSize: Size in pixels of the template patch that is used to compute weights.
/// Should be odd. Recommended value 7 pixels
/// * searchWindowSize: Size in pixels of the window that is used to compute weighted average for
/// given pixel. Should be odd. Affect performance linearly: greater searchWindowsSize - greater
/// denoising time. Recommended value 21 pixels
/// * h: Parameter regulating filter strength for luminance component. Bigger h value perfectly
/// removes noise but also removes image details, smaller h value preserves details but also preserves
/// some noise.
/// * hColor: The same as h but for color components.
/// 
/// The function converts images to CIELAB colorspace and then separately denoise L and AB components
/// with given h parameters using fastNlMeansDenoisingMulti function.
///
/// ## C++ default parameters:
/// * h: 3
/// * h_color: 3
/// * template_window_size: 7
/// * search_window_size: 21
pub fn fast_nl_means_denoising_colored_multi(src_imgs: &types::VectorOfMat, dst: &mut core::Mat, img_to_denoise_index: i32, temporal_window_size: i32, h: f32, h_color: f32, template_window_size: i32, search_window_size: i32) -> Result<()> {
    unsafe { sys::cv_photo_cv_fastNlMeansDenoisingColoredMulti_VectorOfMat_srcImgs_Mat_dst_int_imgToDenoiseIndex_int_temporalWindowSize_float_h_float_hColor_int_templateWindowSize_int_searchWindowSize(src_imgs.as_raw_VectorOfMat(), dst.as_raw_Mat(), img_to_denoise_index, temporal_window_size, h, h_color, template_window_size, search_window_size) }.into_result()
}

// identifier: cv_fastNlMeansDenoisingColored_Mat_src_Mat_dst_float_h_float_hColor_int_templateWindowSize_int_searchWindowSize
/// Modification of fastNlMeansDenoising function for colored images
/// 
/// ## Parameters
/// * src: Input 8-bit 3-channel image.
/// * dst: Output image with the same size and type as src .
/// * templateWindowSize: Size in pixels of the template patch that is used to compute weights.
/// Should be odd. Recommended value 7 pixels
/// * searchWindowSize: Size in pixels of the window that is used to compute weighted average for
/// given pixel. Should be odd. Affect performance linearly: greater searchWindowsSize - greater
/// denoising time. Recommended value 21 pixels
/// * h: Parameter regulating filter strength for luminance component. Bigger h value perfectly
/// removes noise but also removes image details, smaller h value preserves details but also preserves
/// some noise
/// * hColor: The same as h but for color components. For most images value equals 10
/// will be enough to remove colored noise and do not distort colors
/// 
/// The function converts image to CIELAB colorspace and then separately denoise L and AB components
/// with given h parameters using fastNlMeansDenoising function.
///
/// ## C++ default parameters:
/// * h: 3
/// * h_color: 3
/// * template_window_size: 7
/// * search_window_size: 21
pub fn fast_nl_means_denoising_color(src: &core::Mat, dst: &mut core::Mat, h: f32, h_color: f32, template_window_size: i32, search_window_size: i32) -> Result<()> {
    unsafe { sys::cv_photo_cv_fastNlMeansDenoisingColored_Mat_src_Mat_dst_float_h_float_hColor_int_templateWindowSize_int_searchWindowSize(src.as_raw_Mat(), dst.as_raw_Mat(), h, h_color, template_window_size, search_window_size) }.into_result()
}

// identifier: cv_fastNlMeansDenoisingMulti_VectorOfMat_srcImgs_Mat_dst_int_imgToDenoiseIndex_int_temporalWindowSize_VectorOffloat_h_int_templateWindowSize_int_searchWindowSize_int_normType
/// Modification of fastNlMeansDenoising function for images sequence where consecutive images have been
/// captured in small period of time. For example video. This version of the function is for grayscale
/// images or for manual manipulation with colorspaces. For more details see
/// <http://citeseerx.ist.psu.edu/viewdoc/summary?doi=10.1.1.131.6394>
/// 
/// ## Parameters
/// * srcImgs: Input 8-bit or 16-bit (only with NORM_L1) 1-channel,
/// 2-channel, 3-channel or 4-channel images sequence. All images should
/// have the same type and size.
/// * imgToDenoiseIndex: Target image to denoise index in srcImgs sequence
/// * temporalWindowSize: Number of surrounding images to use for target image denoising. Should
/// be odd. Images from imgToDenoiseIndex - temporalWindowSize / 2 to
/// imgToDenoiseIndex - temporalWindowSize / 2 from srcImgs will be used to denoise
/// srcImgs[imgToDenoiseIndex] image.
/// * dst: Output image with the same size and type as srcImgs images.
/// * templateWindowSize: Size in pixels of the template patch that is used to compute weights.
/// Should be odd. Recommended value 7 pixels
/// * searchWindowSize: Size in pixels of the window that is used to compute weighted average for
/// given pixel. Should be odd. Affect performance linearly: greater searchWindowsSize - greater
/// denoising time. Recommended value 21 pixels
/// * h: Array of parameters regulating filter strength, either one
/// parameter applied to all channels or one per channel in dst. Big h value
/// perfectly removes noise but also removes image details, smaller h
/// value preserves details but also preserves some noise
/// * normType: Type of norm used for weight calculation. Can be either NORM_L2 or NORM_L1
///
/// ## C++ default parameters:
/// * template_window_size: 7
/// * search_window_size: 21
/// * norm_type: NORM_L2
pub fn fast_nl_means_denoising_multi(src_imgs: &types::VectorOfMat, dst: &mut core::Mat, img_to_denoise_index: i32, temporal_window_size: i32, h: &types::VectorOffloat, template_window_size: i32, search_window_size: i32, norm_type: i32) -> Result<()> {
    unsafe { sys::cv_photo_cv_fastNlMeansDenoisingMulti_VectorOfMat_srcImgs_Mat_dst_int_imgToDenoiseIndex_int_temporalWindowSize_VectorOffloat_h_int_templateWindowSize_int_searchWindowSize_int_normType(src_imgs.as_raw_VectorOfMat(), dst.as_raw_Mat(), img_to_denoise_index, temporal_window_size, h.as_raw_VectorOffloat(), template_window_size, search_window_size, norm_type) }.into_result()
}

// identifier: cv_fastNlMeansDenoisingMulti_VectorOfMat_srcImgs_Mat_dst_int_imgToDenoiseIndex_int_temporalWindowSize_float_h_int_templateWindowSize_int_searchWindowSize
/// Modification of fastNlMeansDenoising function for images sequence where consecutive images have been
/// captured in small period of time. For example video. This version of the function is for grayscale
/// images or for manual manipulation with colorspaces. For more details see
/// <http://citeseerx.ist.psu.edu/viewdoc/summary?doi=10.1.1.131.6394>
/// 
/// ## Parameters
/// * srcImgs: Input 8-bit 1-channel, 2-channel, 3-channel or
/// 4-channel images sequence. All images should have the same type and
/// size.
/// * imgToDenoiseIndex: Target image to denoise index in srcImgs sequence
/// * temporalWindowSize: Number of surrounding images to use for target image denoising. Should
/// be odd. Images from imgToDenoiseIndex - temporalWindowSize / 2 to
/// imgToDenoiseIndex - temporalWindowSize / 2 from srcImgs will be used to denoise
/// srcImgs[imgToDenoiseIndex] image.
/// * dst: Output image with the same size and type as srcImgs images.
/// * templateWindowSize: Size in pixels of the template patch that is used to compute weights.
/// Should be odd. Recommended value 7 pixels
/// * searchWindowSize: Size in pixels of the window that is used to compute weighted average for
/// given pixel. Should be odd. Affect performance linearly: greater searchWindowsSize - greater
/// denoising time. Recommended value 21 pixels
/// * h: Parameter regulating filter strength. Bigger h value
/// perfectly removes noise but also removes image details, smaller h
/// value preserves details but also preserves some noise
///
/// ## C++ default parameters:
/// * h: 3
/// * template_window_size: 7
/// * search_window_size: 21
pub fn fast_nl_means_denoising_multi_1(src_imgs: &types::VectorOfMat, dst: &mut core::Mat, img_to_denoise_index: i32, temporal_window_size: i32, h: f32, template_window_size: i32, search_window_size: i32) -> Result<()> {
    unsafe { sys::cv_photo_cv_fastNlMeansDenoisingMulti_VectorOfMat_srcImgs_Mat_dst_int_imgToDenoiseIndex_int_temporalWindowSize_float_h_int_templateWindowSize_int_searchWindowSize(src_imgs.as_raw_VectorOfMat(), dst.as_raw_Mat(), img_to_denoise_index, temporal_window_size, h, template_window_size, search_window_size) }.into_result()
}

// identifier: cv_fastNlMeansDenoising_Mat_src_Mat_dst_VectorOffloat_h_int_templateWindowSize_int_searchWindowSize_int_normType
/// Perform image denoising using Non-local Means Denoising algorithm
/// <http://www.ipol.im/pub/algo/bcm_non_local_means_denoising/> with several computational
/// optimizations. Noise expected to be a gaussian white noise
/// 
/// ## Parameters
/// * src: Input 8-bit or 16-bit (only with NORM_L1) 1-channel,
/// 2-channel, 3-channel or 4-channel image.
/// * dst: Output image with the same size and type as src .
/// * templateWindowSize: Size in pixels of the template patch that is used to compute weights.
/// Should be odd. Recommended value 7 pixels
/// * searchWindowSize: Size in pixels of the window that is used to compute weighted average for
/// given pixel. Should be odd. Affect performance linearly: greater searchWindowsSize - greater
/// denoising time. Recommended value 21 pixels
/// * h: Array of parameters regulating filter strength, either one
/// parameter applied to all channels or one per channel in dst. Big h value
/// perfectly removes noise but also removes image details, smaller h
/// value preserves details but also preserves some noise
/// * normType: Type of norm used for weight calculation. Can be either NORM_L2 or NORM_L1
/// 
/// This function expected to be applied to grayscale images. For colored images look at
/// fastNlMeansDenoisingColored. Advanced usage of this functions can be manual denoising of colored
/// image in different colorspaces. Such approach is used in fastNlMeansDenoisingColored by converting
/// image to CIELAB colorspace and then separately denoise L and AB components with different h
/// parameter.
///
/// ## C++ default parameters:
/// * template_window_size: 7
/// * search_window_size: 21
/// * norm_type: NORM_L2
pub fn fast_nl_means_denoising_vec(src: &core::Mat, dst: &mut core::Mat, h: &types::VectorOffloat, template_window_size: i32, search_window_size: i32, norm_type: i32) -> Result<()> {
    unsafe { sys::cv_photo_cv_fastNlMeansDenoising_Mat_src_Mat_dst_VectorOffloat_h_int_templateWindowSize_int_searchWindowSize_int_normType(src.as_raw_Mat(), dst.as_raw_Mat(), h.as_raw_VectorOffloat(), template_window_size, search_window_size, norm_type) }.into_result()
}

// identifier: cv_fastNlMeansDenoising_Mat_src_Mat_dst_float_h_int_templateWindowSize_int_searchWindowSize
/// Perform image denoising using Non-local Means Denoising algorithm
/// <http://www.ipol.im/pub/algo/bcm_non_local_means_denoising/> with several computational
/// optimizations. Noise expected to be a gaussian white noise
/// 
/// ## Parameters
/// * src: Input 8-bit 1-channel, 2-channel, 3-channel or 4-channel image.
/// * dst: Output image with the same size and type as src .
/// * templateWindowSize: Size in pixels of the template patch that is used to compute weights.
/// Should be odd. Recommended value 7 pixels
/// * searchWindowSize: Size in pixels of the window that is used to compute weighted average for
/// given pixel. Should be odd. Affect performance linearly: greater searchWindowsSize - greater
/// denoising time. Recommended value 21 pixels
/// * h: Parameter regulating filter strength. Big h value perfectly removes noise but also
/// removes image details, smaller h value preserves details but also preserves some noise
/// 
/// This function expected to be applied to grayscale images. For colored images look at
/// fastNlMeansDenoisingColored. Advanced usage of this functions can be manual denoising of colored
/// image in different colorspaces. Such approach is used in fastNlMeansDenoisingColored by converting
/// image to CIELAB colorspace and then separately denoise L and AB components with different h
/// parameter.
///
/// ## C++ default parameters:
/// * h: 3
/// * template_window_size: 7
/// * search_window_size: 21
pub fn fast_nl_means_denoising_window(src: &core::Mat, dst: &mut core::Mat, h: f32, template_window_size: i32, search_window_size: i32) -> Result<()> {
    unsafe { sys::cv_photo_cv_fastNlMeansDenoising_Mat_src_Mat_dst_float_h_int_templateWindowSize_int_searchWindowSize(src.as_raw_Mat(), dst.as_raw_Mat(), h, template_window_size, search_window_size) }.into_result()
}

// identifier: cv_illuminationChange_Mat_src_Mat_mask_Mat_dst_float_alpha_float_beta
/// Applying an appropriate non-linear transformation to the gradient field inside the selection and
/// then integrating back with a Poisson solver, modifies locally the apparent illumination of an image.
/// 
/// ## Parameters
/// * src: Input 8-bit 3-channel image.
/// * mask: Input 8-bit 1 or 3-channel image.
/// * dst: Output image with the same size and type as src.
/// * alpha: Value ranges between 0-2.
/// * beta: Value ranges between 0-2.
/// 
/// This is useful to highlight under-exposed foreground objects or to reduce specular reflections.
///
/// ## C++ default parameters:
/// * alpha: 0.2f
/// * beta: 0.4f
pub fn illumination_change(src: &core::Mat, mask: &core::Mat, dst: &mut core::Mat, alpha: f32, beta: f32) -> Result<()> {
    unsafe { sys::cv_photo_cv_illuminationChange_Mat_src_Mat_mask_Mat_dst_float_alpha_float_beta(src.as_raw_Mat(), mask.as_raw_Mat(), dst.as_raw_Mat(), alpha, beta) }.into_result()
}

// identifier: cv_inpaint_Mat_src_Mat_inpaintMask_Mat_dst_double_inpaintRadius_int_flags
/// Restores the selected region in an image using the region neighborhood.
/// 
/// ## Parameters
/// * src: Input 8-bit, 16-bit unsigned or 32-bit float 1-channel or 8-bit 3-channel image.
/// * inpaintMask: Inpainting mask, 8-bit 1-channel image. Non-zero pixels indicate the area that
/// needs to be inpainted.
/// * dst: Output image with the same size and type as src .
/// * inpaintRadius: Radius of a circular neighborhood of each point inpainted that is considered
/// by the algorithm.
/// * flags: Inpainting method that could be cv::INPAINT_NS or cv::INPAINT_TELEA
/// 
/// The function reconstructs the selected image area from the pixel near the area boundary. The
/// function may be used to remove dust and scratches from a scanned photo, or to remove undesirable
/// objects from still images or video. See <http://en.wikipedia.org/wiki/Inpainting> for more details.
/// 
/// 
/// Note:
/// *   An example using the inpainting technique can be found at
/// opencv_source_code/samples/cpp/inpaint.cpp
/// *   (Python) An example using the inpainting technique can be found at
/// opencv_source_code/samples/python/inpaint.py
pub fn inpaint(src: &core::Mat, inpaint_mask: &core::Mat, dst: &mut core::Mat, inpaint_radius: f64, flags: i32) -> Result<()> {
    unsafe { sys::cv_photo_cv_inpaint_Mat_src_Mat_inpaintMask_Mat_dst_double_inpaintRadius_int_flags(src.as_raw_Mat(), inpaint_mask.as_raw_Mat(), dst.as_raw_Mat(), inpaint_radius, flags) }.into_result()
}

// identifier: cv_pencilSketch_Mat_src_Mat_dst1_Mat_dst2_float_sigma_s_float_sigma_r_float_shade_factor
/// Pencil-like non-photorealistic line drawing
/// 
/// ## Parameters
/// * src: Input 8-bit 3-channel image.
/// * dst1: Output 8-bit 1-channel image.
/// * dst2: Output image with the same size and type as src.
/// * sigma_s: %Range between 0 to 200.
/// * sigma_r: %Range between 0 to 1.
/// * shade_factor: %Range between 0 to 0.1.
///
/// ## C++ default parameters:
/// * sigma_s: 60
/// * sigma_r: 0.07f
/// * shade_factor: 0.02f
pub fn pencil_sketch(src: &core::Mat, dst1: &mut core::Mat, dst2: &mut core::Mat, sigma_s: f32, sigma_r: f32, shade_factor: f32) -> Result<()> {
    unsafe { sys::cv_photo_cv_pencilSketch_Mat_src_Mat_dst1_Mat_dst2_float_sigma_s_float_sigma_r_float_shade_factor(src.as_raw_Mat(), dst1.as_raw_Mat(), dst2.as_raw_Mat(), sigma_s, sigma_r, shade_factor) }.into_result()
}

// identifier: cv_seamlessClone_Mat_src_Mat_dst_Mat_mask_Point_p_Mat_blend_int_flags
/// Image editing tasks concern either global changes (color/intensity corrections, filters,
/// deformations) or local changes concerned to a selection. Here we are interested in achieving local
/// changes, ones that are restricted to a region manually selected (ROI), in a seamless and effortless
/// manner. The extent of the changes ranges from slight distortions to complete replacement by novel
/// content [PM03](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_PM03) .
/// 
/// ## Parameters
/// * src: Input 8-bit 3-channel image.
/// * dst: Input 8-bit 3-channel image.
/// * mask: Input 8-bit 1 or 3-channel image.
/// * p: Point in dst image where object is placed.
/// * blend: Output image with the same size and type as dst.
/// * flags: Cloning method that could be cv::NORMAL_CLONE, cv::MIXED_CLONE or cv::MONOCHROME_TRANSFER
pub fn seamless_clone(src: &core::Mat, dst: &core::Mat, mask: &core::Mat, p: core::Point, blend: &mut core::Mat, flags: i32) -> Result<()> {
    unsafe { sys::cv_photo_cv_seamlessClone_Mat_src_Mat_dst_Mat_mask_Point_p_Mat_blend_int_flags(src.as_raw_Mat(), dst.as_raw_Mat(), mask.as_raw_Mat(), p, blend.as_raw_Mat(), flags) }.into_result()
}

// identifier: cv_stylization_Mat_src_Mat_dst_float_sigma_s_float_sigma_r
/// Stylization aims to produce digital imagery with a wide variety of effects not focused on
/// photorealism. Edge-aware filters are ideal for stylization, as they can abstract regions of low
/// contrast while preserving, or enhancing, high-contrast features.
/// 
/// ## Parameters
/// * src: Input 8-bit 3-channel image.
/// * dst: Output image with the same size and type as src.
/// * sigma_s: %Range between 0 to 200.
/// * sigma_r: %Range between 0 to 1.
///
/// ## C++ default parameters:
/// * sigma_s: 60
/// * sigma_r: 0.45f
pub fn stylization(src: &core::Mat, dst: &mut core::Mat, sigma_s: f32, sigma_r: f32) -> Result<()> {
    unsafe { sys::cv_photo_cv_stylization_Mat_src_Mat_dst_float_sigma_s_float_sigma_r(src.as_raw_Mat(), dst.as_raw_Mat(), sigma_s, sigma_r) }.into_result()
}

// identifier: cv_textureFlattening_Mat_src_Mat_mask_Mat_dst_float_low_threshold_float_high_threshold_int_kernel_size
/// By retaining only the gradients at edge locations, before integrating with the Poisson solver, one
/// washes out the texture of the selected region, giving its contents a flat aspect. Here Canny Edge %Detector is used.
/// 
/// ## Parameters
/// * src: Input 8-bit 3-channel image.
/// * mask: Input 8-bit 1 or 3-channel image.
/// * dst: Output image with the same size and type as src.
/// * low_threshold: %Range from 0 to 100.
/// * high_threshold: Value \> 100.
/// * kernel_size: The size of the Sobel kernel to be used.
/// 
/// 
/// Note:
/// The algorithm assumes that the color of the source image is close to that of the destination. This
/// assumption means that when the colors don't match, the source image color gets tinted toward the
/// color of the destination image.
///
/// ## C++ default parameters:
/// * low_threshold: 30
/// * high_threshold: 45
/// * kernel_size: 3
pub fn texture_flattening(src: &core::Mat, mask: &core::Mat, dst: &mut core::Mat, low_threshold: f32, high_threshold: f32, kernel_size: i32) -> Result<()> {
    unsafe { sys::cv_photo_cv_textureFlattening_Mat_src_Mat_mask_Mat_dst_float_low_threshold_float_high_threshold_int_kernel_size(src.as_raw_Mat(), mask.as_raw_Mat(), dst.as_raw_Mat(), low_threshold, high_threshold, kernel_size) }.into_result()
}

// Generating impl for trait cv::AlignExposures (trait)
/// The base class for algorithms that align images of the same scene with different exposures
pub trait AlignExposures : core::Algorithm {
    #[doc(hidden)] fn as_raw_AlignExposures(&self) -> *mut c_void;
    // identifier: cv_AlignExposures_process_VectorOfMat_src_VectorOfMat_dst_Mat_times_Mat_response
    /// Aligns images
    /// 
    /// ## Parameters
    /// * src: vector of input images
    /// * dst: vector of aligned images
    /// * times: vector of exposure time values for each image
    /// * response: 256x1 matrix with inverse camera response function for each pixel value, it should
    /// have the same number of channels as images.
    fn process(&mut self, src: &types::VectorOfMat, dst: &types::VectorOfMat, times: &core::Mat, response: &core::Mat) -> Result<()> {
        unsafe { sys::cv_photo_cv_AlignExposures_process_VectorOfMat_src_VectorOfMat_dst_Mat_times_Mat_response(self.as_raw_AlignExposures(), src.as_raw_VectorOfMat(), dst.as_raw_VectorOfMat(), times.as_raw_Mat(), response.as_raw_Mat()) }.into_result()
    }
    
}

impl<'a> AlignExposures + 'a {

}

// Generating impl for trait cv::AlignMTB (trait)
/// This algorithm converts images to median threshold bitmaps (1 for pixels brighter than median
/// luminance and 0 otherwise) and than aligns the resulting bitmaps using bit operations.
/// 
/// It is invariant to exposure, so exposure values and camera response are not necessary.
/// 
/// In this implementation new image regions are filled with zeros.
/// 
/// For more information see [GW03](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_GW03) .
pub trait AlignMTB : crate::photo::AlignExposures {
    #[doc(hidden)] fn as_raw_AlignMTB(&self) -> *mut c_void;
    // identifier: cv_AlignMTB_process_VectorOfMat_src_VectorOfMat_dst_Mat_times_Mat_response
    fn process_with_response(&mut self, src: &types::VectorOfMat, dst: &types::VectorOfMat, times: &core::Mat, response: &core::Mat) -> Result<()> {
        unsafe { sys::cv_photo_cv_AlignMTB_process_VectorOfMat_src_VectorOfMat_dst_Mat_times_Mat_response(self.as_raw_AlignMTB(), src.as_raw_VectorOfMat(), dst.as_raw_VectorOfMat(), times.as_raw_Mat(), response.as_raw_Mat()) }.into_result()
    }
    
    // identifier: cv_AlignMTB_process_VectorOfMat_src_VectorOfMat_dst
    /// Short version of process, that doesn't take extra arguments.
    /// 
    /// ## Parameters
    /// * src: vector of input images
    /// * dst: vector of aligned images
    fn process(&mut self, src: &types::VectorOfMat, dst: &types::VectorOfMat) -> Result<()> {
        unsafe { sys::cv_photo_cv_AlignMTB_process_VectorOfMat_src_VectorOfMat_dst(self.as_raw_AlignMTB(), src.as_raw_VectorOfMat(), dst.as_raw_VectorOfMat()) }.into_result()
    }
    
    // identifier: cv_AlignMTB_calculateShift_Mat_img0_Mat_img1
    /// Calculates shift between two images, i. e. how to shift the second image to correspond it with the
    /// first.
    /// 
    /// ## Parameters
    /// * img0: first image
    /// * img1: second image
    fn calculate_shift(&mut self, img0: &core::Mat, img1: &core::Mat) -> Result<core::Point> {
        unsafe { sys::cv_photo_cv_AlignMTB_calculateShift_Mat_img0_Mat_img1(self.as_raw_AlignMTB(), img0.as_raw_Mat(), img1.as_raw_Mat()) }.into_result()
    }
    
    // identifier: cv_AlignMTB_shiftMat_Mat_src_Mat_dst_Point_shift
    /// Helper function, that shift Mat filling new regions with zeros.
    /// 
    /// ## Parameters
    /// * src: input image
    /// * dst: result image
    /// * shift: shift value
    fn shift_mat(&mut self, src: &core::Mat, dst: &mut core::Mat, shift: core::Point) -> Result<()> {
        unsafe { sys::cv_photo_cv_AlignMTB_shiftMat_Mat_src_Mat_dst_Point_shift(self.as_raw_AlignMTB(), src.as_raw_Mat(), dst.as_raw_Mat(), shift) }.into_result()
    }
    
    // identifier: cv_AlignMTB_computeBitmaps_Mat_img_Mat_tb_Mat_eb
    /// Computes median threshold and exclude bitmaps of given image.
    /// 
    /// ## Parameters
    /// * img: input image
    /// * tb: median threshold bitmap
    /// * eb: exclude bitmap
    fn compute_bitmaps(&mut self, img: &core::Mat, tb: &mut core::Mat, eb: &mut core::Mat) -> Result<()> {
        unsafe { sys::cv_photo_cv_AlignMTB_computeBitmaps_Mat_img_Mat_tb_Mat_eb(self.as_raw_AlignMTB(), img.as_raw_Mat(), tb.as_raw_Mat(), eb.as_raw_Mat()) }.into_result()
    }
    
    // identifier: cv_AlignMTB_getMaxBits_const
    fn get_max_bits(&self) -> Result<i32> {
        unsafe { sys::cv_photo_cv_AlignMTB_getMaxBits_const(self.as_raw_AlignMTB()) }.into_result()
    }
    
    // identifier: cv_AlignMTB_setMaxBits_int_max_bits
    fn set_max_bits(&mut self, max_bits: i32) -> Result<()> {
        unsafe { sys::cv_photo_cv_AlignMTB_setMaxBits_int_max_bits(self.as_raw_AlignMTB(), max_bits) }.into_result()
    }
    
    // identifier: cv_AlignMTB_getExcludeRange_const
    fn get_exclude_range(&self) -> Result<i32> {
        unsafe { sys::cv_photo_cv_AlignMTB_getExcludeRange_const(self.as_raw_AlignMTB()) }.into_result()
    }
    
    // identifier: cv_AlignMTB_setExcludeRange_int_exclude_range
    fn set_exclude_range(&mut self, exclude_range: i32) -> Result<()> {
        unsafe { sys::cv_photo_cv_AlignMTB_setExcludeRange_int_exclude_range(self.as_raw_AlignMTB(), exclude_range) }.into_result()
    }
    
    // identifier: cv_AlignMTB_getCut_const
    fn get_cut(&self) -> Result<bool> {
        unsafe { sys::cv_photo_cv_AlignMTB_getCut_const(self.as_raw_AlignMTB()) }.into_result()
    }
    
    // identifier: cv_AlignMTB_setCut_bool_value
    fn set_cut(&mut self, value: bool) -> Result<()> {
        unsafe { sys::cv_photo_cv_AlignMTB_setCut_bool_value(self.as_raw_AlignMTB(), value) }.into_result()
    }
    
}

impl<'a> AlignMTB + 'a {

}

// Generating impl for trait cv::CalibrateCRF (trait)
/// The base class for camera response calibration algorithms.
pub trait CalibrateCRF : core::Algorithm {
    #[doc(hidden)] fn as_raw_CalibrateCRF(&self) -> *mut c_void;
    // identifier: cv_CalibrateCRF_process_VectorOfMat_src_Mat_dst_Mat_times
    /// Recovers inverse camera response.
    /// 
    /// ## Parameters
    /// * src: vector of input images
    /// * dst: 256x1 matrix with inverse camera response function
    /// * times: vector of exposure time values for each image
    fn process(&mut self, src: &types::VectorOfMat, dst: &mut core::Mat, times: &core::Mat) -> Result<()> {
        unsafe { sys::cv_photo_cv_CalibrateCRF_process_VectorOfMat_src_Mat_dst_Mat_times(self.as_raw_CalibrateCRF(), src.as_raw_VectorOfMat(), dst.as_raw_Mat(), times.as_raw_Mat()) }.into_result()
    }
    
}

impl<'a> CalibrateCRF + 'a {

}

// Generating impl for trait cv::CalibrateDebevec (trait)
/// Inverse camera response function is extracted for each brightness value by minimizing an objective
/// function as linear system. Objective function is constructed using pixel values on the same position
/// in all images, extra term is added to make the result smoother.
/// 
/// For more information see [DM97](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_DM97) .
pub trait CalibrateDebevec : crate::photo::CalibrateCRF {
    #[doc(hidden)] fn as_raw_CalibrateDebevec(&self) -> *mut c_void;
    // identifier: cv_CalibrateDebevec_getLambda_const
    fn get_lambda(&self) -> Result<f32> {
        unsafe { sys::cv_photo_cv_CalibrateDebevec_getLambda_const(self.as_raw_CalibrateDebevec()) }.into_result()
    }
    
    // identifier: cv_CalibrateDebevec_setLambda_float_lambda
    fn set_lambda(&mut self, lambda: f32) -> Result<()> {
        unsafe { sys::cv_photo_cv_CalibrateDebevec_setLambda_float_lambda(self.as_raw_CalibrateDebevec(), lambda) }.into_result()
    }
    
    // identifier: cv_CalibrateDebevec_getSamples_const
    fn get_samples(&self) -> Result<i32> {
        unsafe { sys::cv_photo_cv_CalibrateDebevec_getSamples_const(self.as_raw_CalibrateDebevec()) }.into_result()
    }
    
    // identifier: cv_CalibrateDebevec_setSamples_int_samples
    fn set_samples(&mut self, samples: i32) -> Result<()> {
        unsafe { sys::cv_photo_cv_CalibrateDebevec_setSamples_int_samples(self.as_raw_CalibrateDebevec(), samples) }.into_result()
    }
    
    // identifier: cv_CalibrateDebevec_getRandom_const
    fn get_random(&self) -> Result<bool> {
        unsafe { sys::cv_photo_cv_CalibrateDebevec_getRandom_const(self.as_raw_CalibrateDebevec()) }.into_result()
    }
    
    // identifier: cv_CalibrateDebevec_setRandom_bool_random
    fn set_random(&mut self, random: bool) -> Result<()> {
        unsafe { sys::cv_photo_cv_CalibrateDebevec_setRandom_bool_random(self.as_raw_CalibrateDebevec(), random) }.into_result()
    }
    
}

impl<'a> CalibrateDebevec + 'a {

}

// Generating impl for trait cv::CalibrateRobertson (trait)
/// Inverse camera response function is extracted for each brightness value by minimizing an objective
/// function as linear system. This algorithm uses all image pixels.
/// 
/// For more information see [RB99](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_RB99) .
pub trait CalibrateRobertson : crate::photo::CalibrateCRF {
    #[doc(hidden)] fn as_raw_CalibrateRobertson(&self) -> *mut c_void;
    // identifier: cv_CalibrateRobertson_getMaxIter_const
    fn get_max_iter(&self) -> Result<i32> {
        unsafe { sys::cv_photo_cv_CalibrateRobertson_getMaxIter_const(self.as_raw_CalibrateRobertson()) }.into_result()
    }
    
    // identifier: cv_CalibrateRobertson_setMaxIter_int_max_iter
    fn set_max_iter(&mut self, max_iter: i32) -> Result<()> {
        unsafe { sys::cv_photo_cv_CalibrateRobertson_setMaxIter_int_max_iter(self.as_raw_CalibrateRobertson(), max_iter) }.into_result()
    }
    
    // identifier: cv_CalibrateRobertson_getThreshold_const
    fn get_threshold(&self) -> Result<f32> {
        unsafe { sys::cv_photo_cv_CalibrateRobertson_getThreshold_const(self.as_raw_CalibrateRobertson()) }.into_result()
    }
    
    // identifier: cv_CalibrateRobertson_setThreshold_float_threshold
    fn set_threshold(&mut self, threshold: f32) -> Result<()> {
        unsafe { sys::cv_photo_cv_CalibrateRobertson_setThreshold_float_threshold(self.as_raw_CalibrateRobertson(), threshold) }.into_result()
    }
    
    // identifier: cv_CalibrateRobertson_getRadiance_const
    fn get_radiance(&self) -> Result<core::Mat> {
        unsafe { sys::cv_photo_cv_CalibrateRobertson_getRadiance_const(self.as_raw_CalibrateRobertson()) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
}

impl<'a> CalibrateRobertson + 'a {

}

// Generating impl for trait cv::MergeDebevec (trait)
/// The resulting HDR image is calculated as weighted average of the exposures considering exposure
/// values and camera response.
/// 
/// For more information see [DM97](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_DM97) .
pub trait MergeDebevec : crate::photo::MergeExposures {
    #[doc(hidden)] fn as_raw_MergeDebevec(&self) -> *mut c_void;
    // identifier: cv_MergeDebevec_process_VectorOfMat_src_Mat_dst_Mat_times_Mat_response
    fn process_with_response(&mut self, src: &types::VectorOfMat, dst: &mut core::Mat, times: &core::Mat, response: &core::Mat) -> Result<()> {
        unsafe { sys::cv_photo_cv_MergeDebevec_process_VectorOfMat_src_Mat_dst_Mat_times_Mat_response(self.as_raw_MergeDebevec(), src.as_raw_VectorOfMat(), dst.as_raw_Mat(), times.as_raw_Mat(), response.as_raw_Mat()) }.into_result()
    }
    
    // identifier: cv_MergeDebevec_process_VectorOfMat_src_Mat_dst_Mat_times
    fn process(&mut self, src: &types::VectorOfMat, dst: &mut core::Mat, times: &core::Mat) -> Result<()> {
        unsafe { sys::cv_photo_cv_MergeDebevec_process_VectorOfMat_src_Mat_dst_Mat_times(self.as_raw_MergeDebevec(), src.as_raw_VectorOfMat(), dst.as_raw_Mat(), times.as_raw_Mat()) }.into_result()
    }
    
}

impl<'a> MergeDebevec + 'a {

}

// Generating impl for trait cv::MergeExposures (trait)
/// The base class algorithms that can merge exposure sequence to a single image.
pub trait MergeExposures : core::Algorithm {
    #[doc(hidden)] fn as_raw_MergeExposures(&self) -> *mut c_void;
    // identifier: cv_MergeExposures_process_VectorOfMat_src_Mat_dst_Mat_times_Mat_response
    /// Merges images.
    /// 
    /// ## Parameters
    /// * src: vector of input images
    /// * dst: result image
    /// * times: vector of exposure time values for each image
    /// * response: 256x1 matrix with inverse camera response function for each pixel value, it should
    /// have the same number of channels as images.
    fn process(&mut self, src: &types::VectorOfMat, dst: &mut core::Mat, times: &core::Mat, response: &core::Mat) -> Result<()> {
        unsafe { sys::cv_photo_cv_MergeExposures_process_VectorOfMat_src_Mat_dst_Mat_times_Mat_response(self.as_raw_MergeExposures(), src.as_raw_VectorOfMat(), dst.as_raw_Mat(), times.as_raw_Mat(), response.as_raw_Mat()) }.into_result()
    }
    
}

impl<'a> MergeExposures + 'a {

}

// Generating impl for trait cv::MergeMertens (trait)
/// Pixels are weighted using contrast, saturation and well-exposedness measures, than images are
/// combined using laplacian pyramids.
/// 
/// The resulting image weight is constructed as weighted average of contrast, saturation and
/// well-exposedness measures.
/// 
/// The resulting image doesn't require tonemapping and can be converted to 8-bit image by multiplying
/// by 255, but it's recommended to apply gamma correction and/or linear tonemapping.
/// 
/// For more information see [MK07](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_MK07) .
pub trait MergeMertens : crate::photo::MergeExposures {
    #[doc(hidden)] fn as_raw_MergeMertens(&self) -> *mut c_void;
    // identifier: cv_MergeMertens_process_VectorOfMat_src_Mat_dst_Mat_times_Mat_response
    fn process_with_response(&mut self, src: &types::VectorOfMat, dst: &mut core::Mat, times: &core::Mat, response: &core::Mat) -> Result<()> {
        unsafe { sys::cv_photo_cv_MergeMertens_process_VectorOfMat_src_Mat_dst_Mat_times_Mat_response(self.as_raw_MergeMertens(), src.as_raw_VectorOfMat(), dst.as_raw_Mat(), times.as_raw_Mat(), response.as_raw_Mat()) }.into_result()
    }
    
    // identifier: cv_MergeMertens_process_VectorOfMat_src_Mat_dst
    /// Short version of process, that doesn't take extra arguments.
    /// 
    /// ## Parameters
    /// * src: vector of input images
    /// * dst: result image
    fn process(&mut self, src: &types::VectorOfMat, dst: &mut core::Mat) -> Result<()> {
        unsafe { sys::cv_photo_cv_MergeMertens_process_VectorOfMat_src_Mat_dst(self.as_raw_MergeMertens(), src.as_raw_VectorOfMat(), dst.as_raw_Mat()) }.into_result()
    }
    
    // identifier: cv_MergeMertens_getContrastWeight_const
    fn get_contrast_weight(&self) -> Result<f32> {
        unsafe { sys::cv_photo_cv_MergeMertens_getContrastWeight_const(self.as_raw_MergeMertens()) }.into_result()
    }
    
    // identifier: cv_MergeMertens_setContrastWeight_float_contrast_weiht
    fn set_contrast_weight(&mut self, contrast_weiht: f32) -> Result<()> {
        unsafe { sys::cv_photo_cv_MergeMertens_setContrastWeight_float_contrast_weiht(self.as_raw_MergeMertens(), contrast_weiht) }.into_result()
    }
    
    // identifier: cv_MergeMertens_getSaturationWeight_const
    fn get_saturation_weight(&self) -> Result<f32> {
        unsafe { sys::cv_photo_cv_MergeMertens_getSaturationWeight_const(self.as_raw_MergeMertens()) }.into_result()
    }
    
    // identifier: cv_MergeMertens_setSaturationWeight_float_saturation_weight
    fn set_saturation_weight(&mut self, saturation_weight: f32) -> Result<()> {
        unsafe { sys::cv_photo_cv_MergeMertens_setSaturationWeight_float_saturation_weight(self.as_raw_MergeMertens(), saturation_weight) }.into_result()
    }
    
    // identifier: cv_MergeMertens_getExposureWeight_const
    fn get_exposure_weight(&self) -> Result<f32> {
        unsafe { sys::cv_photo_cv_MergeMertens_getExposureWeight_const(self.as_raw_MergeMertens()) }.into_result()
    }
    
    // identifier: cv_MergeMertens_setExposureWeight_float_exposure_weight
    fn set_exposure_weight(&mut self, exposure_weight: f32) -> Result<()> {
        unsafe { sys::cv_photo_cv_MergeMertens_setExposureWeight_float_exposure_weight(self.as_raw_MergeMertens(), exposure_weight) }.into_result()
    }
    
}

impl<'a> MergeMertens + 'a {

}

// Generating impl for trait cv::MergeRobertson (trait)
/// The resulting HDR image is calculated as weighted average of the exposures considering exposure
/// values and camera response.
/// 
/// For more information see [RB99](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_RB99) .
pub trait MergeRobertson : crate::photo::MergeExposures {
    #[doc(hidden)] fn as_raw_MergeRobertson(&self) -> *mut c_void;
    // identifier: cv_MergeRobertson_process_VectorOfMat_src_Mat_dst_Mat_times_Mat_response
    fn process_with_response(&mut self, src: &types::VectorOfMat, dst: &mut core::Mat, times: &core::Mat, response: &core::Mat) -> Result<()> {
        unsafe { sys::cv_photo_cv_MergeRobertson_process_VectorOfMat_src_Mat_dst_Mat_times_Mat_response(self.as_raw_MergeRobertson(), src.as_raw_VectorOfMat(), dst.as_raw_Mat(), times.as_raw_Mat(), response.as_raw_Mat()) }.into_result()
    }
    
    // identifier: cv_MergeRobertson_process_VectorOfMat_src_Mat_dst_Mat_times
    fn process(&mut self, src: &types::VectorOfMat, dst: &mut core::Mat, times: &core::Mat) -> Result<()> {
        unsafe { sys::cv_photo_cv_MergeRobertson_process_VectorOfMat_src_Mat_dst_Mat_times(self.as_raw_MergeRobertson(), src.as_raw_VectorOfMat(), dst.as_raw_Mat(), times.as_raw_Mat()) }.into_result()
    }
    
}

impl<'a> MergeRobertson + 'a {

}

// Generating impl for trait cv::Tonemap (trait)
/// Base class for tonemapping algorithms - tools that are used to map HDR image to 8-bit range.
pub trait Tonemap : core::Algorithm {
    #[doc(hidden)] fn as_raw_Tonemap(&self) -> *mut c_void;
    // identifier: cv_Tonemap_process_Mat_src_Mat_dst
    /// Tonemaps image
    /// 
    /// ## Parameters
    /// * src: source image - CV_32FC3 Mat (float 32 bits 3 channels)
    /// * dst: destination image - CV_32FC3 Mat with values in [0, 1] range
    fn process(&mut self, src: &core::Mat, dst: &mut core::Mat) -> Result<()> {
        unsafe { sys::cv_photo_cv_Tonemap_process_Mat_src_Mat_dst(self.as_raw_Tonemap(), src.as_raw_Mat(), dst.as_raw_Mat()) }.into_result()
    }
    
    // identifier: cv_Tonemap_getGamma_const
    fn get_gamma(&self) -> Result<f32> {
        unsafe { sys::cv_photo_cv_Tonemap_getGamma_const(self.as_raw_Tonemap()) }.into_result()
    }
    
    // identifier: cv_Tonemap_setGamma_float_gamma
    fn set_gamma(&mut self, gamma: f32) -> Result<()> {
        unsafe { sys::cv_photo_cv_Tonemap_setGamma_float_gamma(self.as_raw_Tonemap(), gamma) }.into_result()
    }
    
}

impl<'a> Tonemap + 'a {

}

// Generating impl for trait cv::TonemapDrago (trait)
/// Adaptive logarithmic mapping is a fast global tonemapping algorithm that scales the image in
/// logarithmic domain.
/// 
/// Since it's a global operator the same function is applied to all the pixels, it is controlled by the
/// bias parameter.
/// 
/// Optional saturation enhancement is possible as described in [FL02](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_FL02) .
/// 
/// For more information see [DM03](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_DM03) .
pub trait TonemapDrago : crate::photo::Tonemap {
    #[doc(hidden)] fn as_raw_TonemapDrago(&self) -> *mut c_void;
    // identifier: cv_TonemapDrago_getSaturation_const
    fn get_saturation(&self) -> Result<f32> {
        unsafe { sys::cv_photo_cv_TonemapDrago_getSaturation_const(self.as_raw_TonemapDrago()) }.into_result()
    }
    
    // identifier: cv_TonemapDrago_setSaturation_float_saturation
    fn set_saturation(&mut self, saturation: f32) -> Result<()> {
        unsafe { sys::cv_photo_cv_TonemapDrago_setSaturation_float_saturation(self.as_raw_TonemapDrago(), saturation) }.into_result()
    }
    
    // identifier: cv_TonemapDrago_getBias_const
    fn get_bias(&self) -> Result<f32> {
        unsafe { sys::cv_photo_cv_TonemapDrago_getBias_const(self.as_raw_TonemapDrago()) }.into_result()
    }
    
    // identifier: cv_TonemapDrago_setBias_float_bias
    fn set_bias(&mut self, bias: f32) -> Result<()> {
        unsafe { sys::cv_photo_cv_TonemapDrago_setBias_float_bias(self.as_raw_TonemapDrago(), bias) }.into_result()
    }
    
}

impl<'a> TonemapDrago + 'a {

}

// Generating impl for trait cv::TonemapMantiuk (trait)
/// This algorithm transforms image to contrast using gradients on all levels of gaussian pyramid,
/// transforms contrast values to HVS response and scales the response. After this the image is
/// reconstructed from new contrast values.
/// 
/// For more information see [MM06](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_MM06) .
pub trait TonemapMantiuk : crate::photo::Tonemap {
    #[doc(hidden)] fn as_raw_TonemapMantiuk(&self) -> *mut c_void;
    // identifier: cv_TonemapMantiuk_getScale_const
    fn get_scale(&self) -> Result<f32> {
        unsafe { sys::cv_photo_cv_TonemapMantiuk_getScale_const(self.as_raw_TonemapMantiuk()) }.into_result()
    }
    
    // identifier: cv_TonemapMantiuk_setScale_float_scale
    fn set_scale(&mut self, scale: f32) -> Result<()> {
        unsafe { sys::cv_photo_cv_TonemapMantiuk_setScale_float_scale(self.as_raw_TonemapMantiuk(), scale) }.into_result()
    }
    
    // identifier: cv_TonemapMantiuk_getSaturation_const
    fn get_saturation(&self) -> Result<f32> {
        unsafe { sys::cv_photo_cv_TonemapMantiuk_getSaturation_const(self.as_raw_TonemapMantiuk()) }.into_result()
    }
    
    // identifier: cv_TonemapMantiuk_setSaturation_float_saturation
    fn set_saturation(&mut self, saturation: f32) -> Result<()> {
        unsafe { sys::cv_photo_cv_TonemapMantiuk_setSaturation_float_saturation(self.as_raw_TonemapMantiuk(), saturation) }.into_result()
    }
    
}

impl<'a> TonemapMantiuk + 'a {

}

// Generating impl for trait cv::TonemapReinhard (trait)
/// This is a global tonemapping operator that models human visual system.
/// 
/// Mapping function is controlled by adaptation parameter, that is computed using light adaptation and
/// color adaptation.
/// 
/// For more information see [RD05](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_RD05) .
pub trait TonemapReinhard : crate::photo::Tonemap {
    #[doc(hidden)] fn as_raw_TonemapReinhard(&self) -> *mut c_void;
    // identifier: cv_TonemapReinhard_getIntensity_const
    fn get_intensity(&self) -> Result<f32> {
        unsafe { sys::cv_photo_cv_TonemapReinhard_getIntensity_const(self.as_raw_TonemapReinhard()) }.into_result()
    }
    
    // identifier: cv_TonemapReinhard_setIntensity_float_intensity
    fn set_intensity(&mut self, intensity: f32) -> Result<()> {
        unsafe { sys::cv_photo_cv_TonemapReinhard_setIntensity_float_intensity(self.as_raw_TonemapReinhard(), intensity) }.into_result()
    }
    
    // identifier: cv_TonemapReinhard_getLightAdaptation_const
    fn get_light_adaptation(&self) -> Result<f32> {
        unsafe { sys::cv_photo_cv_TonemapReinhard_getLightAdaptation_const(self.as_raw_TonemapReinhard()) }.into_result()
    }
    
    // identifier: cv_TonemapReinhard_setLightAdaptation_float_light_adapt
    fn set_light_adaptation(&mut self, light_adapt: f32) -> Result<()> {
        unsafe { sys::cv_photo_cv_TonemapReinhard_setLightAdaptation_float_light_adapt(self.as_raw_TonemapReinhard(), light_adapt) }.into_result()
    }
    
    // identifier: cv_TonemapReinhard_getColorAdaptation_const
    fn get_color_adaptation(&self) -> Result<f32> {
        unsafe { sys::cv_photo_cv_TonemapReinhard_getColorAdaptation_const(self.as_raw_TonemapReinhard()) }.into_result()
    }
    
    // identifier: cv_TonemapReinhard_setColorAdaptation_float_color_adapt
    fn set_color_adaptation(&mut self, color_adapt: f32) -> Result<()> {
        unsafe { sys::cv_photo_cv_TonemapReinhard_setColorAdaptation_float_color_adapt(self.as_raw_TonemapReinhard(), color_adapt) }.into_result()
    }
    
}

impl<'a> TonemapReinhard + 'a {

}

