import cv2
import numpy as np

cv2.namedWindow("preview")
vc = cv2.VideoCapture(0)

sift = cv2.xfeatures2d.SIFT_create()

if vc.isOpened():  # try to get the first frame
    rval, frame = vc.read()
else:
    rval = False

gray = cv2.cvtColor(frame, cv2.COLOR_BGR2GRAY)
# init_kp = sift.detect(gray, None)
# init_pts = map(lambda x: x.pt, init_kp)

while rval:

    gray = cv2.cvtColor(frame, cv2.COLOR_BGR2GRAY)
    # print gray.size
    # print gray.shape

    kp = sift.detect(gray, None)
    # pts = map(lambda x: x.pt, kp)
    img = cv2.drawKeypoints(gray, kp, frame, flags=cv2.DRAW_MATCHES_FLAGS_DRAW_RICH_KEYPOINTS)

    # ret, corners = cv2.findChessboardCorners(gray, (7, 6), None)
    # print ret
    # print corners
    # if ret:
    #     cv2.drawChessboardCorners(img, (7, 6), corners, retval)

    # print cv2.stereoCalibrate(init_pts, init_pts, pts, gray.size, np.ndarray(3, 3), [], [], gray.size)

    cv2.imshow("preview", img)
    rval, frame = vc.read()
    cv2.imwrite('webcamtest.jpg', img)
    # key = cv2.waitKey(20)
    # if key == 27:  # exit on ESC
    #     break
cv2.destroyWindow("preview")
